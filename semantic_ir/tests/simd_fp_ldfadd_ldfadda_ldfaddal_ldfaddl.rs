/// LDFADD, LDFADDA, LDFADDAL, LDFADDL: LDFADD, LDFADDA, LDFADDAL, LDFADDL
/// Stanford source: simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl_instruction() -> Instruction {
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

    // LDFADD, LDFADDA, LDFADDAL, LDFADDL: LDFADD, LDFADDA, LDFADDAL, LDFADDL
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd,
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
            mnemonic: "LDFADD, LDFADDA, LDFADDAL, LDFADDL".to_string(),
            encoding: Some("LDFADD, LDFADDA, LDFADDAL, LDFADDL".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl_validates() {
    let instruction = simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl_instruction();
    validate_instruction(&instruction).expect("LDFADD, LDFADDA, LDFADDAL, LDFADDL should validate");
}

#[test]
fn simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl_uses_vector_map2() {
    let instruction = simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDFADD, LDFADDA, LDFADDAL, LDFADDL");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPAdd);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W32 }));
}
