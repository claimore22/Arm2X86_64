/// FMAX/FMIN (immediate, SVE predicated) - Floating-point max/min with immediate
/// Stanford sources: arm64_md_clean/fmax_z_p_zs.md, arm64_md_clean/fmin_z_p_zs.md
///
/// FMAX (immediate): Zdn = max(Zdn, 0.0/1.0) (active), Zdn (inactive) - destructive
/// FMIN (immediate): Zdn = min(Zdn, 0.0/1.0) (active), Zdn (inactive) - destructive
/// Immediate is either +0.0 (i1='0') or +1.0 (i1='1')
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fmax_imm_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate value: FPOne (1.0) - i1='1'
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

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMax,
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
            mnemonic: "FMAX".to_string(),
            encoding: Some("Floating-point maximum with immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmax_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fmin_imm_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate value: FPOne (1.0)
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

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMin,
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
            mnemonic: "FMIN".to_string(),
            encoding: Some("Floating-point minimum with immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmin_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fmaxnm_imm_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate value: FPOne (1.0)
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

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMaxNum,
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
            mnemonic: "FMAXNM".to_string(),
            encoding: Some("Floating-point maximum number with immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxnm_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fminnm_imm_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate value: FPOne (1.0)
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

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMinNum,
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
            mnemonic: "FMINNM".to_string(),
            encoding: Some("Floating-point minimum number with immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminnm_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmax_imm_sve_predicated_validates() {
    let instruction = fmax_imm_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMAX immediate SVE should validate");
}

#[test]
fn fmax_imm_sve_predicated_structure() {
    let instruction = fmax_imm_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FMAX immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMax);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    let Expr::SVESpecial { op: sv_op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fmin_imm_sve_predicated_validates() {
    let instruction = fmin_imm_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMIN immediate SVE should validate");
}

#[test]
fn fmin_imm_sve_predicated_structure() {
    let instruction = fmin_imm_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FMIN immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMin);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    let Expr::SVESpecial { op: sv_op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fmaxnm_imm_sve_predicated_validates() {
    let instruction = fmaxnm_imm_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMAXNM immediate SVE should validate");
}

#[test]
fn fmaxnm_imm_sve_predicated_structure() {
    let instruction = fmaxnm_imm_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FMAXNM immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMaxNum);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    let Expr::SVESpecial { op: sv_op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fminnm_imm_sve_predicated_validates() {
    let instruction = fminnm_imm_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMINNM immediate SVE should validate");
}

#[test]
fn fminnm_imm_sve_predicated_structure() {
    let instruction = fminnm_imm_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FMINNM immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMinNum);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    let Expr::SVESpecial { op: sv_op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}