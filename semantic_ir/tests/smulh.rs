/// SMULH (Signed multiply high) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/smulh.md
///
/// Xd = (Sext(Xn) * Sext(Xm))[127:64].
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn smulh_instruction() -> Instruction {
    let a = Expr::SignExtend { value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }), to: BitWidth::W128 };
    let b = Expr::SignExtend { value: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }), to: BitWidth::W128 };
    let product = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(a),
        right: Box::new(b),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W128)),
    };
    let result = Expr::Extract {
        value: Box::new(product),
        lsb: 64,
        width: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SMULH".to_string(), encoding: Some("Signed multiply high".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "smulh.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn smulh_validates() {
    validate_instruction(&smulh_instruction()).expect("should validate");
}
