/// VSUB (scalar): Scalar subtract
/// Stanford source: vsub_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vsub_scalar_instruction() -> Instruction {
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
    // VSUB (scalar): Scalar subtract
    let result = Expr::ScalarFPBinary {
        op: semantic_ir::primitives::ScalarFPBinOp::FPSub,
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
            mnemonic: "VSUB (scalar)".to_string(),
            encoding: Some("Scalar subtract".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vsub_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vsub_scalar_validates() {
    let instruction = vsub_scalar_instruction();
    validate_instruction(&instruction).expect("VSUB (scalar) should validate");
}

#[test]
fn vsub_scalar_uses_scalar_fp_sub() {
    let instruction = vsub_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalarFPBinary { op, left, right, ty } = value else {
        panic!("expected ScalarFPBinary for VSUB (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::ScalarFPBinOp::FPSub);
    assert_eq!(*ty, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W64 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W64 }));
}