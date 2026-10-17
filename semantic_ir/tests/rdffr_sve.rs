/// RDFFR (predicated) - Return predicate of successfully loaded elements
/// Stanford source: arm64_md_clean/rdffr_p_p_f.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn rdffr_predicated_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    // RDFFR predicated: result = FFR AND mask
    let ffr = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Rdffr,
        predicate: None,
        operands: vec![],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::And,
        left: Box::new(ffr),
        right: Box::new(predicate),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "RDFFR (predicated)".to_string(),
            encoding: Some("Return predicate of successfully loaded elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rdffr_p_p_f.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// RDFFR (unpredicated) - Read the first-fault register
/// Stanford source: arm64_md_clean/rdffr_p_f.md
fn rdffr_unpredicated_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    // RDFFR unpredicated: result = FFR
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Rdffr,
        predicate: None,
        operands: vec![],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "RDFFR (unpredicated)".to_string(),
            encoding: Some("Read the first-fault register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rdffr_p_f.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// RDFFRS - Return predicate of successfully loaded elements, setting the condition flags
/// Stanford source: arm64_md_clean/rdffrs_p_p_f.md
fn rdffrs_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    // RDFFRS: result = FFR AND mask (with condition flags set)
    let ffr = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Rdffrs,
        predicate: None,
        operands: vec![],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::And,
        left: Box::new(ffr),
        right: Box::new(predicate),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "RDFFRS".to_string(),
            encoding: Some("Return predicate of successfully loaded elements, setting the condition flags".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rdffrs_p_p_f.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rdffr_predicated_sve_validates() {
    let instruction = rdffr_predicated_sve_instruction();
    validate_instruction(&instruction).expect("RDFFR (predicated) SVE should validate");
}

#[test]
fn rdffr_unpredicated_sve_validates() {
    let instruction = rdffr_unpredicated_sve_instruction();
    validate_instruction(&instruction).expect("RDFFR (unpredicated) SVE should validate");
}

#[test]
fn rdffrs_sve_validates() {
    let instruction = rdffrs_sve_instruction();
    validate_instruction(&instruction).expect("RDFFRS SVE should validate");
}

#[test]
fn rdffr_predicated_sve_structure() {
    let instruction = rdffr_predicated_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap2 { op, left, right, pl, .. } = value else {
        panic!("expected PredicateMap2 for RDFFR predicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::And);
    assert_eq!(*pl, BitWidth::W32);
    let Expr::SVESpecial { op: ffr_op, .. } = left.as_ref() else {
        panic!("expected SVESpecial for FFR");
    };
    assert_eq!(*ffr_op, semantic_ir::primitives::SVESpecialOp::Rdffr);
    matches!(right.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn rdffr_unpredicated_sve_structure() {
    let instruction = rdffr_unpredicated_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, .. } = value else {
        panic!("expected SVESpecial for RDFFR unpredicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Rdffr);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 0);
    assert_eq!(*vl, BitWidth::W256);
}

#[test]
fn rdffrs_sve_structure() {
    let instruction = rdffrs_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap2 { op, left, right, pl, .. } = value else {
        panic!("expected PredicateMap2 for RDFFRS");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::And);
    assert_eq!(*pl, BitWidth::W32);
    let Expr::SVESpecial { op: ffr_op, .. } = left.as_ref() else {
        panic!("expected SVESpecial for FFR");
    };
    assert_eq!(*ffr_op, semantic_ir::primitives::SVESpecialOp::Rdffrs);
    matches!(right.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}