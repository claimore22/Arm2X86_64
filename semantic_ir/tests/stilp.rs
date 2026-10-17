/// STILP (Store-release ordered pair of registers) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stilp.md
///
/// Pair store: two consecutive MemWrite at X(n)+imm7 (and +element size).
/// Release ordering (FEAT_LRCPC3).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stilp_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let addr0 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 16, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let addr1 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(addr0.clone()),
        right: Box::new(Expr::Const { value: 8, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![
        Stmt::MemWrite {
            address: addr0,
            value: Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 },
            size: MemSize::Bits64,
            access: MemAccess::Write,
            ordering: MemOrdering::Release,
        },
        Stmt::MemWrite {
            address: addr1,
            value: Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 },
            size: MemSize::Bits64,
            access: MemAccess::Write,
            ordering: MemOrdering::Release,
        },
    ];

    Instruction {
        id: InstructionId { mnemonic: "STILP".to_string(), encoding: Some("Store-release ordered pair".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC3".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stilp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stilp_validates() {
    validate_instruction(&stilp_instruction()).expect("should validate");
}

#[test]
fn stilp_stores_pair() {
    let instruction = stilp_instruction();
    let n = instruction
        .semantics
        .iter()
        .filter(|s| matches!(s, Stmt::MemWrite { .. }))
        .count();
    assert_eq!(n, 2);
}
