/// SBFM (Signed Bitfield Move) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sbfm.md
/// This is the base instruction for ASR (immediate), SBFIZ, SBFX, SXTB, SXTH, SXTW.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn sbfm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let src = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let imm6 = BitWidth::new(6).unwrap();
    let r = Expr::Const {
        value: 8,
        ty: imm6,
    };
    let _s = Expr::Const {
        value: 15,
        ty: imm6,
    };

    let wmask = Expr::Const {
        value: 0x000000000000FFFFu128,
        ty: BitWidth::W64,
    };
    let tmask = Expr::Const {
        value: 0x000000000000FFFFu128,
        ty: BitWidth::W64,
    };

    // bot = ROR(src, r) AND wmask
    let ror_src = Expr::Shift {
        kind: ShiftKind::Ror,
        value: Box::new(src.clone()),
        amount: Box::new(r),
        ty: BitWidth::W64,
    };

    let bot = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(ror_src),
        right: Box::new(wmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // top = Replicate(src[s])
    // src[s] is bit 15 of the source (the MSB of the bitfield)
    // Extract bit 15 as a 1-bit bitvector, then replicate it
    let src_bit_s = Expr::Extract {
        value: Box::new(src),
        lsb: 15,
        width: BitWidth::W1,
    };

    // Replicate creates a vector by repeating the element
    // For 64-bit: Replicate{64}(src[s]) creates 64 copies of the bit
    let top = Expr::Replicate {
        value: Box::new(src_bit_s),
        count: 64,
        element_width: BitWidth::W1,
    };

    // result = (top AND NOT(tmask)) OR (bot AND tmask)
    let not_tmask = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(tmask.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let top_and_not_tmask = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(top),
        right: Box::new(not_tmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let bot_and_tmask = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(bot),
        right: Box::new(tmask.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(top_and_not_tmask),
        right: Box::new(bot_and_tmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SBFM".to_string(),
            encoding: Some("Signed bitfield move".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sbfm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sbfm_validates() {
    let instruction = sbfm_instruction();

    validate_instruction(&instruction).expect("SBFM should validate");
}

#[test]
fn sbfm_writes_result() {
    let instruction = sbfm_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn sbfm_uses_replicate_and_sign_extension() {
    let instruction = sbfm_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    // result = (top AND NOT(tmask)) OR (bot AND tmask)
    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Or for final result");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Or);

    // left = top AND NOT(tmask)
    let Expr::Binary { op: op2, left: top_part, right: not_tmask_part, .. } = left.as_ref() else {
        panic!("expected Binary And for top part");
    };

    assert_eq!(*op2, semantic_ir::BinOp::And);

    // top_part should be Replicate
    let Expr::Replicate { value: repl_value, count, element_width } = top_part.as_ref() else {
        panic!("expected Replicate for top");
    };

    assert_eq!(*count, 64);
    assert_eq!(*element_width, BitWidth::W1);

    // repl_value should be Extract (src[s])
    let Expr::Extract { value: ext_value, lsb, width } = repl_value.as_ref() else {
        panic!("expected Extract inside Replicate");
    };

    assert_eq!(*lsb, 15);
    assert_eq!(*width, BitWidth::W1);
    assert!(matches!(ext_value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // right = bot AND tmask
    let Expr::Binary { op: op3, left: bot_part, right: tmask_part, .. } = right.as_ref() else {
        panic!("expected Binary And for bot part");
    };

    assert_eq!(*op3, semantic_ir::BinOp::And);
    assert!(matches!(bot_part.as_ref(), Expr::Binary { .. })); // bot = ROR AND wmask
    assert!(matches!(tmask_part.as_ref(), Expr::Const { .. }));
}