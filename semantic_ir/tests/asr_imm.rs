/// ASR (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/asr_imm.md
///
/// ASR (immediate) is an alias of SBFM with:
///   immr = 0
///   imms = width - 1 - shift
///   N = 0 for 32-bit, 1 for 64-bit
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn asr_imm_instruction() -> Instruction {
    // This test instantiates the instruction at datasize = 64 with shift = 4.
    let datasize = Width::Fixed(BitWidth::W64);
    let shift = 4u64;

    let src = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // ASR (immediate) maps to SBFM with:
    // - immr = 0
    // - imms = width - 1 - shift = 63 - 4 = 59
    // - N = 1 (for 64-bit)
    //
    // The SBFM result for ASR is:
    //   top = Replicate{64}(src[63])  // sign bit
    //   bot = ROR(src, 0) AND wmask   // just src AND wmask
    //   wmask = (1 << (imms + 1)) - 1 = (1 << 60) - 1
    //   result = (top AND NOT(wmask)) OR (bot AND wmask)

    let width_val = 64u64;
    let imms = width_val - 1 - shift; // 59

    // wmask = (1 << (imms + 1)) - 1 = (1 << 60) - 1
    let wmask = Expr::Const {
        value: (1u128 << (imms + 1)) - 1,
        ty: BitWidth::W64,
    };

    // top = Replicate{64}(src[63])
    let src_bit_s = Expr::Extract {
        value: Box::new(src.clone()),
        lsb: 63,
        width: BitWidth::W1,
    };

    let top = Expr::Replicate {
        value: Box::new(src_bit_s),
        count: 64,
        element_width: BitWidth::W1,
    };

    // bot = src AND wmask (ROR by 0 is identity)
    let bot = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(src.clone()),
        right: Box::new(wmask.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // result = (top AND NOT(wmask)) OR (bot AND wmask)
    let not_wmask = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(wmask.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let top_and_not_wmask = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(top),
        right: Box::new(not_wmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let bot_and_wmask = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(bot),
        right: Box::new(wmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(top_and_not_wmask),
        right: Box::new(bot_and_wmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ASR".to_string(),
            encoding: Some("Arithmetic shift right immediate (alias of SBFM)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asr_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asr_imm_validates() {
    let instruction = asr_imm_instruction();

    validate_instruction(&instruction).expect("ASR immediate should validate");
}

#[test]
fn asr_imm_writes_result() {
    let instruction = asr_imm_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn asr_imm_uses_replicate_and_sign_extension() {
    let instruction = asr_imm_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    // result = (top AND NOT(wmask)) OR (bot AND wmask)
    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Or for final result");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Or);

    // left = top AND NOT(wmask)
    let Expr::Binary { op: op2, left: top_part, right: not_wmask_part, .. } = left.as_ref() else {
        panic!("expected Binary And for top part");
    };

    assert_eq!(*op2, semantic_ir::BinOp::And);

    // top_part should be Replicate
    let Expr::Replicate { value: repl_value, count, element_width } = top_part.as_ref() else {
        panic!("expected Replicate for top");
    };

    assert_eq!(*count, 64);
    assert_eq!(*element_width, BitWidth::W1);

    // repl_value should be Extract (src[63])
    let Expr::Extract { value: ext_value, lsb, width } = repl_value.as_ref() else {
        panic!("expected Extract inside Replicate");
    };

    assert_eq!(*lsb, 63);
    assert_eq!(*width, BitWidth::W1);
    assert!(matches!(ext_value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // right = bot AND wmask
    let Expr::Binary { op: op3, left: bot_part, right: wmask_part, .. } = right.as_ref() else {
        panic!("expected Binary And for bot part");
    };

    assert_eq!(*op3, semantic_ir::BinOp::And);
    assert!(matches!(bot_part.as_ref(), Expr::Binary { .. })); // bot = src AND wmask
    assert!(matches!(wmask_part.as_ref(), Expr::Const { .. }));
}