/// UMNEGL (alias of UMSUBL) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/umnegl_umsubl.md
///
/// Xd = 0 - Zext(Wn)*Zext(Wm).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn umnegl_instruction() -> Instruction {
    let a = Expr::ZeroExtend { value: Box::new(Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W32 }), to: BitWidth::W64 };
    let b = Expr::ZeroExtend { value: Box::new(Expr::ReadReg { reg: Reg::W(3), width: BitWidth::W32 }), to: BitWidth::W64 };
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
        id: InstructionId { mnemonic: "UMNEGL".to_string(), encoding: Some("__TITLE__".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "umnegl_umsubl.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn umnegl_validates() {
    validate_instruction(&umnegl_instruction()).expect("should validate");
}
