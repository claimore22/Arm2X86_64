/// SCVTF (vector, fixed-point): SCVTF (vector, fixed-point)
/// Stanford source: simd_fp_scvtf_vector_fixed_point.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_scvtf_vector_fixed_point_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // SCVTF (vector, fixed-point): SCVTF (vector, fixed-point)
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTZS,
        operand: Box::new(operand),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SCVTF (vector, fixed-point)".to_string(),
            encoding: Some("SCVTF (vector, fixed-point)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_scvtf_vector_fixed_point.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_scvtf_vector_fixed_point_validates() {
    let instruction = simd_fp_scvtf_vector_fixed_point_instruction();
    validate_instruction(&instruction).expect("SCVTF (vector, fixed-point) should validate");
}

#[test]
fn simd_fp_scvtf_vector_fixed_point_uses_vector_map() {
    let instruction = simd_fp_scvtf_vector_fixed_point_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for SCVTF (vector, fixed-point)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTZS);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
