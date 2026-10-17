/// LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML: LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML
/// Stanford source: simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml_instruction() -> Instruction {
    let datasize = BitWidth::W32;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W32,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W32,
    };

    // LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML: LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMax,
        left: Box::new(left),
        right: Box::new(right),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML".to_string(),
            encoding: Some("LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml_validates() {
    let instruction = simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml_instruction();
    validate_instruction(&instruction).expect("LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML should validate");
}

#[test]
fn simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml_uses_vector_map2() {
    let instruction = simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPMax);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W32 }));
}
