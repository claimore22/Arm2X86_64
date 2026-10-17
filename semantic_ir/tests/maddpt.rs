/// MADDPT: multiply-add checked pointer (FEAT_CPA). Xd = PointerAddCheck(Xn*Xm + Xa, Xn).
/// Stanford source: arm64_md_clean/maddpt.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn maddpt_instruction() -> Instruction {
    let product = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let sum = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(product),
        right: Box::new(Expr::ReadReg { reg: Reg::X(4), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let result = Expr::PointerAddCheck {
        value: Box::new(sum),
        base: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "MADDPT".to_string(), encoding: Some("Multiply-add checked pointer".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "maddpt.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn maddpt_validates() {
    validate_instruction(&maddpt_instruction()).expect("should validate");
}
