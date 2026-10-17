/// SABD (SVE predicated) - Signed absolute difference
/// Stanford source: arm64_md_clean/sabd_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn sabd_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let left = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let right = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fabd,
        predicate: Box::new(predicate),
        left: Box::new(left),
        right: Box::new(right),
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 1, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABD (SVE predicated)".to_string(),
            encoding: Some("Signed absolute difference (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabd_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// SABD (SVE unpredicated) - Signed absolute difference
fn sabd_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let left = Expr::ReadScalableVecReg { reg: 1, vl };
    let right = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fabd,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        left: Box::new(left),
        right: Box::new(right),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 1, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABD (SVE unpredicated)".to_string(),
            encoding: Some("Signed absolute difference (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabd_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sabd_sve_predicated_validates() {
    let instruction = sabd_sve_predicated_instruction();
    validate_instruction(&instruction).expect("SABD (SVE predicated) should validate");
}

#[test]
fn sabd_sve_unpredicated_validates() {
    let instruction = sabd_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("SABD (SVE unpredicated) should validate");
}

#[test]
fn sabd_sve_predicated_structure() {
    let instruction = sabd_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for SABD predicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fabd);
    assert!(matches!(&**predicate, Expr::ReadPredReg { .. }));
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}