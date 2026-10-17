/// INDEX (SVE unpredicated) - Create index vector
/// Stanford sources:
///   - arm64_md_clean/index_z_ii.md (immediates)
///   - arm64_md_clean/index_z_ir.md (immediate, scalar)
///   - arm64_md_clean/index_z_ri.md (scalar, immediate)
///   - arm64_md_clean/index_z_rr.md (scalars)
///
/// INDEX variants:
///   INDEX (immediates): Zd = [imm1 + e*imm2]
///   INDEX (immediate, scalar): Zd = [imm + e*X[m]]
///   INDEX (scalar, immediate): Zd = [X[n] + e*imm]
///   INDEX (scalars): Zd = [X[n] + e*X[m]]
/// All unpredicated.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn index_immediates_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 8, 16, 32, 64

    // Immediate operands: imm1 = 5, imm2 = 3 (example values)
    let imm1 = Expr::Const { value: 5i32 as u128, ty: BitWidth::W32 };
    let imm2 = Expr::Const { value: 3i32 as u128, ty: BitWidth::W32 };

    // INDEX (immediates): Zd = [imm1 + e*imm2]
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: None,
        operands: vec![Box::new(imm1), Box::new(imm2)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "INDEX".to_string(),
            encoding: Some("Create index starting from and incremented by immediate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "index_z_ii.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn index_imm_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    // Immediate start value: 5
    let imm = Expr::Const { value: 5i32 as u128, ty: BitWidth::W32 };
    // Scalar increment from X register
    let scalar = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    // INDEX (immediate, scalar): Zd = [imm + e*X[m]]
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: None,
        operands: vec![Box::new(imm), Box::new(scalar)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "INDEX".to_string(),
            encoding: Some("Create index starting from immediate and incremented by scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "index_z_ir.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn index_scalar_imm_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    // Scalar start from X register
    let scalar = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    // Immediate increment: 3
    let imm = Expr::Const { value: 3i32 as u128, ty: BitWidth::W32 };

    // INDEX (scalar, immediate): Zd = [X[n] + e*imm]
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: None,
        operands: vec![Box::new(scalar), Box::new(imm)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "INDEX".to_string(),
            encoding: Some("Create index starting from scalar and incremented by immediate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "index_z_ri.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn index_scalars_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    // Both operands from X registers
    let scalar1 = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let scalar2 = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };

    // INDEX (scalars): Zd = [X[n] + e*X[m]]
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: None,
        operands: vec![Box::new(scalar1), Box::new(scalar2)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "INDEX".to_string(),
            encoding: Some("Create index starting from and incremented by scalars".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "index_z_rr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn index_immediates_sve_validates() {
    let instruction = index_immediates_sve_instruction();
    validate_instruction(&instruction).expect("INDEX immediates SVE should validate");
}

#[test]
fn index_immediates_sve_structure() {
    let instruction = index_immediates_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for INDEX immediates");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::Const { value: 5, ty: BitWidth::W32 });
    matches!(operands[1].as_ref(), Expr::Const { value: 3, ty: BitWidth::W32 });
}

#[test]
fn index_imm_scalar_sve_validates() {
    let instruction = index_imm_scalar_sve_instruction();
    validate_instruction(&instruction).expect("INDEX imm/scalar SVE should validate");
}

#[test]
fn index_imm_scalar_sve_structure() {
    let instruction = index_imm_scalar_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for INDEX imm/scalar");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::Const { value: 5, ty: BitWidth::W32 });
    matches!(operands[1].as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
}

#[test]
fn index_scalar_imm_sve_validates() {
    let instruction = index_scalar_imm_sve_instruction();
    validate_instruction(&instruction).expect("INDEX scalar/imm SVE should validate");
}

#[test]
fn index_scalar_imm_sve_structure() {
    let instruction = index_scalar_imm_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for INDEX scalar/imm");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
    matches!(operands[1].as_ref(), Expr::Const { value: 3, ty: BitWidth::W32 });
}

#[test]
fn index_scalars_sve_validates() {
    let instruction = index_scalars_sve_instruction();
    validate_instruction(&instruction).expect("INDEX scalars SVE should validate");
}

#[test]
fn index_scalars_sve_structure() {
    let instruction = index_scalars_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for INDEX scalars");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
    matches!(operands[1].as_ref(), Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 });
}