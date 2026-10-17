/// VNEG (scalar): Scalar negate
/// Stanford source: vneg_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vneg_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W64;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W64,
    };
    // VNEG (scalar): Scalar negate
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Neg,
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
            mnemonic: "VNEG (scalar)".to_string(),
            encoding: Some("Scalar negate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vneg_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vneg_scalar_validates() {
    let instruction = vneg_scalar_instruction();
    validate_instruction(&instruction).expect("VNEG (scalar) should validate");
}

#[test]
fn vneg_scalar_uses_vector_map() {
    let instruction = vneg_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VNEG (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Neg);
    assert_eq!(*datasize, BitWidth::W64);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W64 }));
}
