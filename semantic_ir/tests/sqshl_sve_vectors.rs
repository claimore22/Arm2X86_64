/// SQSHL (vectors, predicated) - Signed saturating shift left
/// Stanford source: arm64_md_clean/sqshl_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn sqshl_sve_vectors_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - first source / destination
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zm - second source (shift amounts)

    // SQSHL: signed saturating shift left by vector elements
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Sqshl,
        predicate: Box::new(predicate),
        left: Box::new(operand1.clone()),      // Zdn
        middle: Box::new(operand2),    // Zm
        right: Box::new(operand1.clone()), // Zdn (destination)
        destination: Box::new(operand1),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQSHL (vectors)".to_string(),
            encoding: Some("Signed saturating shift left (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqshl_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqshl_sve_vectors_validates() {
    let instruction = sqshl_sve_vectors_instruction();
    validate_instruction(&instruction).expect("SQSHL SVE vectors should validate");
}

#[test]
fn sqshl_sve_vectors_structure() {
    let instruction = sqshl_sve_vectors_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for SQSHL vectors");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Sqshl);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}