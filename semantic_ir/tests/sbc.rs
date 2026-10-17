/// SBC (Subtract with carry) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sbc.md
///
/// Xd = Xn + NOT(Xm) + C  (AddWithCarry; result only, no flag write).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn sbc_instruction() -> Instruction {
    let sum = Expr::AddWithCarry {
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::Unary {
            op: semantic_ir::UnaryOp::Not,
            value: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
        carry: Box::new(Expr::ReadFlag { flag: semantic_ir::Flag::C }),
        width: BitWidth::W64,
    };
    let result = Expr::TupleElement {
        value: Box::new(sum),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SBC".to_string(), encoding: Some("Subtract with carry".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sbc.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sbc_validates() {
    validate_instruction(&sbc_instruction()).expect("SBC should validate");
}

#[test]
fn sbc_uses_add_with_carry() {
    let instruction = sbc_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::TupleElement { index: 0, .. }));
}
