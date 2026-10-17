/// NGC: alias of SBC (negate with carry). Xd = 0 - Xn - (1 - C).
/// Stanford source: arm64_md_clean/ngc_sbc.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn ngc_instruction() -> Instruction {
    // NGC = SBC Xd, XZR, Xn: Xd = 0 + NOT(Xn) + C  (AddWithCarry)
    let sum = Expr::AddWithCarry {
        left: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        right: Box::new(Expr::Unary {
            op: semantic_ir::UnaryOp::Not,
            value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
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
        id: InstructionId { mnemonic: "NGC".to_string(), encoding: Some("Negate with carry (alias of SBC)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "ngc_sbc.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn ngc_validates() {
    validate_instruction(&ngc_instruction()).expect("should validate");
}
