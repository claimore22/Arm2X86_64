/// VSQRT (scalar): Scalar square root
/// Stanford source: vsqrt_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vsqrt_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W64;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W64,
    };
    // VSQRT (scalar): Scalar square root
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for VSQRT
        operand: Box::new(operand),
        datasize: BitWidth::W64,
        esize: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VSQRT (scalar)".to_string(),
            encoding: Some("Scalar square root".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vsqrt_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vsqrt_scalar_validates() {
    let instruction = vsqrt_scalar_instruction();
    validate_instruction(&instruction).expect("VSQRT (scalar) should validate");
}

#[test]
fn vsqrt_scalar_uses_vector_map() {
    let instruction = vsqrt_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VSQRT (scalar)");
    };

    assert_eq!(*datasize, BitWidth::W64);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W64 }));
}
