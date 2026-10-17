/// SCVTF (scalar SIMD&FP): SCVTF (scalar SIMD&FP)
/// Stanford source: simd_fp_scvtf_scalar_simdfp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_scvtf_scalar_simdfp_instruction() -> Instruction {
    let datasize = BitWidth::W32;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W32,
    };

    // SCVTF (scalar SIMD&FP): SCVTF (scalar SIMD&FP)
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
            mnemonic: "SCVTF (scalar SIMD&FP)".to_string(),
            encoding: Some("SCVTF (scalar SIMD&FP)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_scvtf_scalar_simdfp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_scvtf_scalar_simdfp_validates() {
    let instruction = simd_fp_scvtf_scalar_simdfp_instruction();
    validate_instruction(&instruction).expect("SCVTF (scalar SIMD&FP) should validate");
}

#[test]
fn simd_fp_scvtf_scalar_simdfp_uses_vector_map() {
    let instruction = simd_fp_scvtf_scalar_simdfp_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for SCVTF (scalar SIMD&FP)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTZS);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
}
