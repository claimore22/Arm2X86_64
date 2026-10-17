/// FTMAD, FTSMUL, FTSSEL (SVE) - Floating-point trigonometric multiply-add/select
/// Stanford sources: arm64_md_clean/ftmad_z_zzi.md, arm64_md_clean/ftsmul_z_zz.md, arm64_md_clean/ftssel_z_zz.md
///
/// FTMAD: Floating-point trigonometric multiply-add coefficient
///   Zdn = Zdn * |Zm| + coefficient(imm, sign(Zm)) (destructive on Zdn)
///   Uses hard-wired coefficients for Taylor series approximation of sin/cos
///   imm (0-7) selects coefficient index
///
/// FTSMUL: Floating-point trigonometric starting value
///   Zd = Zn * Zn with sign from Zm[0]
///   Zd = Zn^2 with sign from Zm[0]
///
/// FTSSEL: Floating-point trigonometric select coefficient
///   Selects between Zn and 1.0 based on Zm[0], negates if Zm[1] set
///   Selects x, 1.0, -x, -1.0 for sin/cos quadrant handling
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ftmad_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // Double-precision in example

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // Immediate index (0-7) - broadcast to vector using SVESpecialOp::Dup
    let imm_index = Expr::Const { value: 3, ty: BitWidth::W4 };
    let broadcast_imm = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(imm_index)],
        vl,
        esize: Some(BitWidth::W32), // index is typically small
        imm: None,
    };

    // FTMAD: Zdn = Zdn * |Zm| + coefficient(imm, sign(Zm))
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Ftmad,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),      // Zdn
        middle: Box::new(operand2),    // Zm
        right: Box::new(broadcast_imm), // immediate index 0-7 broadcast
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // unpredicated
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 1, // Zdn
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FTMAD".to_string(),
            encoding: Some("Floating-point trigonometric multiply-add coefficient".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ftmad_z_zzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ftsmul_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    // FTSMUL: Zd = Zn * Zn with sign from Zm[0]
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Ftsmul,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }), // Zd
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // unpredicated
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FTSMUL".to_string(),
            encoding: Some("Floating-point trigonometric starting value".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ftsmul_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ftssel_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    // FTSSEL: selects Zn or 1.0 based on Zm[0], negates if Zm[1] set
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Ftssel,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }), // Zd
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // unpredicated
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FTSSEL".to_string(),
            encoding: Some("Floating-point trigonometric select coefficient".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ftssel_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ftmad_sve_validates() {
    let instruction = ftmad_sve_instruction();
    validate_instruction(&instruction).expect("FTMAD SVE should validate");
}

#[test]
fn ftmad_sve_structure() {
    let instruction = ftmad_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 {
        op,
        left,
        middle,
        right,
        destination,
        vl,
        esize,
        predication,
..
    } = value else {
        panic!("expected ScalableVectorMap3 for FTMAD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Ftmad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    // Right is broadcast immediate (SVESpecial with Dup)
    let Expr::SVESpecial { op: sv_op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::Const { value: 3, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn ftsmul_sve_validates() {
    let instruction = ftsmul_sve_instruction();
    validate_instruction(&instruction).expect("FTSMUL SVE should validate");
}

#[test]
fn ftsmul_sve_structure() {
    let instruction = ftsmul_sve_instruction();

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
        panic!("expected ScalableVectorMap2 for FTSMUL");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Ftsmul);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn ftssel_sve_validates() {
    let instruction = ftssel_sve_instruction();
    validate_instruction(&instruction).expect("FTSSEL SVE should validate");
}

#[test]
fn ftssel_sve_structure() {
    let instruction = ftssel_sve_instruction();

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
        panic!("expected ScalableVectorMap2 for FTSSEL");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Ftssel);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}
