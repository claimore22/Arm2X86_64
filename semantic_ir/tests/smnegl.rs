/// SMNEGL (alias of SMSUBL) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/smnegl_smsubl.md
///
/// Xd = 0 - Sext(Wn)*Sext(Wm): multiply-negate long.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn smnegl_instruction() -> Instruction {
    // Xd = Sub(Xa, Sext(Wn) * Sext(Wm))
    let a = Expr::SignExtend { value: Box::new(Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W32 }), to: BitWidth::W64 };
    let b = Expr::SignExtend { value: Box::new(Expr::ReadReg { reg: Reg::W(3), width: BitWidth::W32 }), to: BitWidth::W64 };
    let product = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(a),
        right: Box::new(b),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(product),
        right: Box::new(Expr::ReadReg { reg: Reg::X(31), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SMNEGL".to_string(), encoding: Some("Signed multiply-negate long".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "smnegl_smsubl.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn smnegl_validates() {
    validate_instruction(&smnegl_instruction()).expect("should validate");
}
