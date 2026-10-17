/// URSHLR (SVE predicated) - Unsigned rounding shift left reversed (predicated)
/// Stanford source: arm64_md_clean/urshrl_z_p_zz.md (assumed)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn urshrl_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let left = Expr::ReadScalableVecReg { reg: 1, vl }; // Zm
    let right = Expr::ReadScalableVecReg { reg: 2, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 2, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Urshrl,
        predicate: Box::new(predicate),
        left: Box::new(left),
        right: Box::new(right),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 2, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "URSHLR".to_string(),
            encoding: Some("Unsigned rounding shift left reversed (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "urshrl_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn urshrl_sve_validates() {
    let instruction = urshrl_sve_instruction();
    validate_instruction(&instruction).expect("URSHLR SVE should validate");
}

#[test]
fn urshrl_sve_structure() {
    let instruction = urshrl_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for URSHLR");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Urshrl);
    matches!(&**predicate, Expr::ReadPredReg { .. });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}