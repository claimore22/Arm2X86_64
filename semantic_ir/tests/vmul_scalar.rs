/// VMUL (scalar): Scalar multiply
/// Stanford source: vmul_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vmul_scalar_instruction() -> Instruction {
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
    // VMUL (scalar): Scalar multiply
    let result = Expr::ScalarFPBinary {
        op: semantic_ir::primitives::ScalarFPBinOp::FPMul,
        left: Box::new(left),
        right: Box::new(right),
        ty: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VMUL (scalar)".to_string(),
            encoding: Some("Scalar multiply".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vmul_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vmul_scalar_validates() {
    let instruction = vmul_scalar_instruction();
    validate_instruction(&instruction).expect("VMUL (scalar) should validate");
}

#[test]
fn vmul_scalar_uses_scalar_fp_mult() {
    let instruction = vmul_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalarFPBinary { op, left, right, ty } = value else {
        panic!("expected ScalarFPBinary for VMUL (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::ScalarFPBinOp::FPMul);
    assert_eq!(*ty, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W64 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W64 }));
}