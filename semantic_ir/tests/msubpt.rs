/// MSUBPT: multiply-subtract checked pointer (FEAT_CPA). Xd = PointerAddCheck(Xa - Xn*Xm, Xn).
/// Stanford source: arm64_md_clean/msubpt.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn msubpt_instruction() -> Instruction {
    let product = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let diff = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(Expr::ReadReg { reg: Reg::X(4), width: BitWidth::W64 }),
        right: Box::new(product),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let result = Expr::PointerAddCheck {
        value: Box::new(diff),
        base: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "MSUBPT".to_string(), encoding: Some("Multiply-subtract checked pointer".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "msubpt.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn msubpt_validates() {
    validate_instruction(&msubpt_instruction()).expect("should validate");
}
