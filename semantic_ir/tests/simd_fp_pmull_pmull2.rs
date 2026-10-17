/// PMULL, PMULL2: PMULL, PMULL2
/// Stanford source: simd_fp_pmull_pmull2.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_pmull_pmull2_instruction() -> Instruction {
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

    // PMULL, PMULL2: PMULL, PMULL2
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul,
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
            mnemonic: "PMULL, PMULL2".to_string(),
            encoding: Some("PMULL, PMULL2".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_pmull_pmull2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_pmull_pmull2_validates() {
    let instruction = simd_fp_pmull_pmull2_instruction();
    validate_instruction(&instruction).expect("PMULL, PMULL2 should validate");
}

#[test]
fn simd_fp_pmull_pmull2_uses_vector_map2() {
    let instruction = simd_fp_pmull_pmull2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for PMULL, PMULL2");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Mul);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W32 }));
}
