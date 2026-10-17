/// FSUBR (SVE) - Floating-point reversed subtract
/// Stanford sources: arm64_md_clean/fsubr_z_p_zz.md, arm64_md_clean/fsubr_z_p_zs.md
///
/// FSUBR (vectors): Zdn = Zm - Zdn (active), Zdn (inactive) - destructive
/// FSUBR (immediate): Zdn = 0.5/1.0 - Zdn (active), Zdn (inactive) - destructive
/// Reverse subtract: imm - Zdn instead of Zdn - imm
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fsubr_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // FSUBR: Zm - Zdn (reversed)
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fsubr,
        predicate: Box::new(predicate),
        left: Box::new(operand2),  // Zm (first operand for reverse subtract)
        right: Box::new(operand1), // Zdn (second operand)
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
            mnemonic: "FSUBR".to_string(),
            encoding: Some("Floating-point reversed subtract (vectors, predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fsubr_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fsubr_sve_immediate_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate value: FPOne (1.0) or FPPointFive (0.5)
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

    // FSUBR immediate: imm - Zdn (reversed)
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fsubr,
        predicate: Box::new(predicate),
        left: Box::new(broadcast_imm),  // immediate (first operand for reverse subtract)
        right: Box::new(operand1),      // Zdn (second operand)
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
            mnemonic: "FSUBR".to_string(),
            encoding: Some("Floating-point reversed subtract immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fsubr_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fsubr_sve_predicated_validates() {
    let instruction = fsubr_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FSUBR SVE predicated should validate");
}

#[test]
fn fsubr_sve_predicated_structure() {
    let instruction = fsubr_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FSUBR predicated");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fsubr);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    // Left is Zm, right is Zdn (reversed)
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fsubr_sve_immediate_validates() {
    let instruction = fsubr_sve_immediate_instruction();
    validate_instruction(&instruction).expect("FSUBR SVE immediate should validate");
}

#[test]
fn fsubr_sve_immediate_structure() {
    let instruction = fsubr_sve_immediate_instruction();

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
        panic!("expected ScalableVectorMap2 for FSUBR immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fsubr);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    // Left is broadcast immediate (SVESpecial with Dup)
    let Expr::SVESpecial { op: sv_op, operands, .. } = left.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
    // Right is Zdn
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}