/// FAMAX, FAMIN (SVE predicated) - Floating-point absolute maximum/minimum
/// Stanford sources: arm64_md_clean/famax_z_p_zz.md, arm64_md_clean/famin_z_p_zz.md
///
/// FAMAX: result = max(|Zdn|, |Zm|) for active elements, Zdn for inactive (destructive)
/// FAMIN: result = min(|Zdn|, |Zm|) for active elements, Zdn for inactive (destructive)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn famax_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FAMax,
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
            mnemonic: "FAMAX".to_string(),
            encoding: Some("Floating-point absolute maximum (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME2".to_string(), "FEAT_FAMINMAX".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "famax_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn famin_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FAMin,
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
            mnemonic: "FAMIN".to_string(),
            encoding: Some("Floating-point absolute minimum (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME2".to_string(), "FEAT_FAMINMAX".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "famin_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn famax_sve_predicated_validates() {
    let instruction = famax_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FAMAX SVE predicated should validate");
}

#[test]
fn famax_sve_predicated_structure() {
    let instruction = famax_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FAMAX");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FAMax);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn famin_sve_predicated_validates() {
    let instruction = famin_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FAMIN SVE predicated should validate");
}

#[test]
fn famin_sve_predicated_structure() {
    let instruction = famin_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FAMIN");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FAMin);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}