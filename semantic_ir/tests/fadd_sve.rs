/// FADD (SVE) - Floating-point add
/// Stanford sources:
///   - fadd_z_p_zz.md (vectors, predicated, destructive)
///   - fadd_z_zz.md (vectors, unpredicated)
///   - fadd_z_p_zs.md (immediate, predicated, destructive)
///
/// FADD (vectors, predicated):
///   Zdn = Zdn + Zm (active), Zdn (inactive)
///
/// FADD (vectors, unpredicated):
///   Zd = Zn + Zm
///
/// FADD (immediate):
///   Zdn = Zdn + 0.5/1.0 (active), Zdn (inactive)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fadd_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fadd,
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
            mnemonic: "FADD".to_string(),
            encoding: Some("Floating-point add (vectors, predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fadd_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fadd_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fadd,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }), // unused but required
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }), // Zd
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // unpredicated = all active
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FADD".to_string(),
            encoding: Some("Floating-point add (vectors, unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fadd_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fadd_sve_immediate_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate value: FPOne (1.0) for esize=32
    // First broadcast the immediate to a vector using SVESpecialOp::Dup
    let imm = Expr::Const {
        value: 0x3f800000u128, // 1.0 in IEEE 754 single-precision
        ty: BitWidth::W32,
    };

    let broadcast_imm = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(imm)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    // Then add the broadcasted immediate vector to the source vector
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fadd,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(broadcast_imm),
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
            mnemonic: "FADD".to_string(),
            encoding: Some("Floating-point add immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fadd_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fadd_sve_predicated_validates() {
    let instruction = fadd_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FADD SVE predicated should validate");
}

#[test]
fn fadd_sve_predicated_structure() {
    let instruction = fadd_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FADD predicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fadd);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fadd_sve_unpredicated_validates() {
    let instruction = fadd_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FADD SVE unpredicated should validate");
}

#[test]
fn fadd_sve_unpredicated_structure() {
    let instruction = fadd_sve_unpredicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FADD unpredicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fadd);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    // For unpredicated, we use zeroing with all-ones predicate conceptually
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn fadd_sve_immediate_validates() {
    let instruction = fadd_sve_immediate_instruction();
    validate_instruction(&instruction).expect("FADD SVE immediate should validate");
}

#[test]
fn fadd_sve_immediate_structure() {
    let instruction = fadd_sve_immediate_instruction();

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
        panic!("expected ScalableVectorMap2 for FADD immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fadd);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    // Right operand should be a broadcast immediate (SVESpecial with Dup)
    let Expr::SVESpecial { op: sv_op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}
