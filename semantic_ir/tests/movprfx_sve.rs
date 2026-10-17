/// MOVPRFX (unpredicated) - Move prefix (unpredicated)
/// Stanford source: arm64_md_clean/movprfx_z_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn movprfx_unpredicated_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Movprfx,
        predicate: None,
        operands: vec![Box::new(operand)],
        vl,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVPRFX (unpredicated)".to_string(),
            encoding: Some("Move prefix (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movprfx_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// MOVPRFX (predicated, merging) - Move prefix (predicated)
/// Stanford source: arm64_md_clean/movprfx_z_p_z.md
fn movprfx_predicated_merging_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Use ScalableVectorMap with identity operation to model predicated copy
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination), vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVPRFX (predicated, merging)".to_string(),
            encoding: Some("Move prefix (predicated, merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movprfx_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// MOVPRFX (predicated, zeroing) - Move prefix (predicated)
fn movprfx_predicated_zeroing_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Use ScalableVectorMap with identity operation to model predicated copy
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVPRFX (predicated, zeroing)".to_string(),
            encoding: Some("Move prefix (predicated, zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movprfx_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn movprfx_unpredicated_sve_validates() {
    let instruction = movprfx_unpredicated_sve_instruction();
    validate_instruction(&instruction).expect("MOVPRFX (unpredicated) SVE should validate");
}

#[test]
fn movprfx_predicated_merging_sve_validates() {
    let instruction = movprfx_predicated_merging_sve_instruction();
    validate_instruction(&instruction).expect("MOVPRFX (predicated, merging) SVE should validate");
}

#[test]
fn movprfx_predicated_zeroing_sve_validates() {
    let instruction = movprfx_predicated_zeroing_sve_instruction();
    validate_instruction(&instruction).expect("MOVPRFX (predicated, zeroing) SVE should validate");
}

#[test]
fn movprfx_unpredicated_sve_structure() {
    let instruction = movprfx_unpredicated_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, .. } = value else {
        panic!("expected SVESpecial for MOVPRFX unpredicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Movprfx);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
}

#[test]
fn movprfx_predicated_merging_sve_structure() {
    let instruction = movprfx_predicated_merging_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap { op, predicate, operand, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap for MOVPRFX predicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Id);
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}