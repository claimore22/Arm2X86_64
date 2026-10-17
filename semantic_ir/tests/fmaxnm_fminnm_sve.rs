/// FMAXNM, FMINNM (SVE predicated) - Floating-point maximum/minimum number
/// Stanford sources: arm64_md_clean/fmaxnm_z_p_zz.md, arm64_md_clean/fminnm_z_p_zz.md
///
/// FMAXNM (vectors): Zdn = maxnum(Zdn, Zm) for active, Zdn for inactive (destructive)
/// FMINNM (vectors): Zdn = minnum(Zdn, Zm) for active, Zdn for inactive (destructive)
///
/// Unlike FMAX/FMIN, these handle NaNs by returning the numeric operand
/// when one operand is NaN and the other is numeric.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fmaxnm_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMaxNum,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 1, // Zdn
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAXNM".to_string(),
            encoding: Some("Floating-point maximum number (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxnm_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fminnm_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMinNum,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 1, // Zdn
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINNM".to_string(),
            encoding: Some("Floating-point minimum number (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminnm_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmaxnm_sve_predicated_validates() {
    let instruction = fmaxnm_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMAXNM SVE predicated should validate");
}

#[test]
fn fmaxnm_sve_predicated_structure() {
    let instruction = fmaxnm_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 {
        op,
        predicate,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap2 for FMAXNM");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMaxNum);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fminnm_sve_predicated_validates() {
    let instruction = fminnm_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMINNM SVE predicated should validate");
}

#[test]
fn fminnm_sve_predicated_structure() {
    let instruction = fminnm_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 {
        op,
        predicate,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap2 for FMINNM");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMinNum);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}