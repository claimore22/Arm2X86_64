/// BFMLS (multiple vectors): BFMLS (multiple vectors)
/// Stanford source: sme_bfmls_multiple_vectors.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sme_bfmls_multiple_vectors_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // BFMLS (multiple vectors): BFMLS (multiple vectors)
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
            mnemonic: "BFMLS (multiple vectors)".to_string(),
            encoding: Some("BFMLS (multiple vectors)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sme_bfmls_multiple_vectors.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sme_bfmls_multiple_vectors_validates() {
    let instruction = sme_bfmls_multiple_vectors_instruction();
    validate_instruction(&instruction).expect("BFMLS (multiple vectors) should validate");
}

#[test]
fn sme_bfmls_multiple_vectors_uses_vector_map2() {
    let instruction = sme_bfmls_multiple_vectors_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for BFMLS (multiple vectors)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Mul);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}
