/// STBFMAXNM, STBFMAXNML: STBFMAXNM, STBFMAXNML
/// Stanford source: simd_fp_stbfmaxnm_stbfmaxnml.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_stbfmaxnm_stbfmaxnml_instruction() -> Instruction {
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

    // STBFMAXNM, STBFMAXNML: STBFMAXNM, STBFMAXNML
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
            mnemonic: "STBFMAXNM, STBFMAXNML".to_string(),
            encoding: Some("STBFMAXNM, STBFMAXNML".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_stbfmaxnm_stbfmaxnml.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_stbfmaxnm_stbfmaxnml_validates() {
    let instruction = simd_fp_stbfmaxnm_stbfmaxnml_instruction();
    validate_instruction(&instruction).expect("STBFMAXNM, STBFMAXNML should validate");
}

#[test]
fn simd_fp_stbfmaxnm_stbfmaxnml_uses_vector_map2() {
    let instruction = simd_fp_stbfmaxnm_stbfmaxnml_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STBFMAXNM, STBFMAXNML");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPMax);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W32 }));
}
