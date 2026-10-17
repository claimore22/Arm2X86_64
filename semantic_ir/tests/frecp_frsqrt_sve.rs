/// FRECPE, FRECPX, FRSQRTE, FRSQRTS, FRECPS (SVE) - Floating-point reciprocal/sqrt estimates
/// Stanford sources:
///   - frecpe (unpredicated)
///   - frecpx (predicated)
///   - frsqrte (unpredicated)
///   - frsqrits (unpredicated)
///   - frecps (unpredicated)
///
/// FRECPE: reciprocal estimate (unpredicated) - 1/x approximation
/// FRECPX: reciprocal exponent (predicated) - extracts exponent for reciprocal
/// FRSQRTE: reciprocal square root estimate (unpredicated) - 1/sqrt(x) approximation
/// FRSQRTS: reciprocal square root step (unpredicated) - Newton-Raphson refinement step
/// FRECPS: reciprocal step (unpredicated) - Newton-Raphson refinement for reciprocal
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn frecpe_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Frecpe,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRECPE".to_string(),
            encoding: Some("Floating-point reciprocal estimate (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frecpe_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn frecpx_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::FRecpX,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination), vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRECPX".to_string(),
            encoding: Some("Floating-point reciprocal exponent (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frecpx_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn frsqrte_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Frsqrte,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRSQRTE".to_string(),
            encoding: Some("Floating-point reciprocal square root estimate (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frsqrte_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn frsqrits_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // FRSQRTS: reciprocal square root step - Newton-Raphson refinement
    // x_{n+1} = x_n * (1.5 - 0.5 * d * x_n^2)
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Frsqrts,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRSQRTS".to_string(),
            encoding: Some("Floating-point reciprocal square root step (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frsqrits_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn frecps_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // FRECPS: reciprocal step - Newton-Raphson refinement
    // x_{n+1} = x_n * (2 - d * x_n)
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Frecps,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRECPS".to_string(),
            encoding: Some("Floating-point reciprocal step (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frecps_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frecpe_sve_unpredicated_validates() {
    let instruction = frecpe_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FRECPE SVE unpredicated should validate");
}

#[test]
fn frecpe_sve_unpredicated_structure() {
    let instruction = frecpe_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        operand,
        destination,
        vl,
        esize,
        predication,
..
    } = value else {
        panic!("expected ScalableVectorMap for FRECPE");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Frecpe);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn frecpx_sve_predicated_validates() {
    let instruction = frecpx_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FRECPX SVE predicated should validate");
}

#[test]
fn frecpx_sve_predicated_structure() {
    let instruction = frecpx_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        predicate,
        operand,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap for FRECPX");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::FRecpX);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn frsqrte_sve_unpredicated_validates() {
    let instruction = frsqrte_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FRSQRTE SVE unpredicated should validate");
}

#[test]
fn frsqrte_sve_unpredicated_structure() {
    let instruction = frsqrte_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        operand,
        destination,
        vl,
        esize,
        predication,
..
    } = value else {
        panic!("expected ScalableVectorMap for FRSQRTE");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Frsqrte);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn frsqrits_sve_unpredicated_validates() {
    let instruction = frsqrits_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FRSQRTS SVE unpredicated should validate");
}

#[test]
fn frsqrits_sve_unpredicated_structure() {
    let instruction = frsqrits_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 {
        op,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
..
    } = value else {
        panic!("expected ScalableVectorMap2 for FRSQRTS");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Frsqrts);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn frecps_sve_unpredicated_validates() {
    let instruction = frecps_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FRECPS SVE unpredicated should validate");
}

#[test]
fn frecps_sve_unpredicated_structure() {
    let instruction = frecps_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 {
        op,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
..
    } = value else {
        panic!("expected ScalableVectorMap2 for FRECPS");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Frecps);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}
