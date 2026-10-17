/// SQSHLR (vectors, predicated) - Signed saturating shift left reversed
/// Stanford source: arm64_md_clean/sqshlr_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn sqshlr_sve_vectors_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zm - first source (values to shift)
    let operand2 = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - second source (shift amounts) / destination

    // SQSHLR: signed saturating shift left reversed (shift amounts in Zdn, values in Zm)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Sqshlr,
        predicate: Box::new(predicate),
        left: Box::new(operand1.clone()),      // Zm - values to shift
        middle: Box::new(operand2.clone()),    // Zdn - shift amounts
        right: Box::new(operand2.clone()), // Zdn (destination)
        destination: Box::new(operand2),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQSHLR (vectors)".to_string(),
            encoding: Some("Signed saturating shift left reversed (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqshlr_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqshlr_sve_vectors_validates() {
    let instruction = sqshlr_sve_vectors_instruction();
    validate_instruction(&instruction).expect("SQSHLR SVE vectors should validate");
}

#[test]
fn sqshlr_sve_vectors_structure() {
    let instruction = sqshlr_sve_vectors_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for SQSHLR vectors");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Sqshlr);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}