/// VFMS (scalar): Scalar floating-point fused multiply-subtract
/// Stanford source: vfms_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vfms_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W64;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W64,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W64,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W64,
    };
    // VFMS (scalar): Scalar floating-point fused multiply-subtract
    let mul = Expr::ScalarFPBinary {
        op: semantic_ir::primitives::ScalarFPBinOp::FPMul,
        left: Box::new(left),
        right: Box::new(right),
        ty: BitWidth::W64,
    };
    let result = Expr::ScalarFPBinary {
        op: semantic_ir::primitives::ScalarFPBinOp::FPSub,
        left: Box::new(addend),
        right: Box::new(mul),
        ty: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VFMS (scalar)".to_string(),
            encoding: Some("Scalar floating-point fused multiply-subtract".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vfms_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vfms_scalar_validates() {
    let instruction = vfms_scalar_instruction();
    validate_instruction(&instruction).expect("VFMS (scalar) should validate");
}

#[test]
fn vfms_scalar_uses_scalar_fms() {
    let instruction = vfms_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalarFPBinary { op, left, right, ty } = value else {
        panic!("expected ScalarFPBinary for VFMS (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::ScalarFPBinOp::FPSub);
    assert_eq!(*ty, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W64 }));

    // Check right is Mul
    let Expr::ScalarFPBinary { op: mul_op, left: mul_left, right: mul_right, ty: mul_ty } = right.as_ref() else {
        panic!("expected ScalarFPBinary for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::ScalarFPBinOp::FPMul);
    assert_eq!(*mul_ty, BitWidth::W64);
    assert!(matches!(mul_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W64 }));
    assert!(matches!(mul_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W64 }));
}