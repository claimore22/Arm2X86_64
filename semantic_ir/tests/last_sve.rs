/// LASTA, LASTB, LASTP (SVE) - Extract last/after-last element, find last true predicate
/// Stanford sources:
///   - arm64_md_clean/lasta_r_p_z.md (scalar)
///   - arm64_md_clean/lasta_v_p_z.md (SIMD&FP scalar)
///   - arm64_md_clean/lastb_r_p_z.md (scalar)
///   - arm64_md_clean/lastb_v_p_z.md (SIMD&FP scalar)
///   - arm64_md_clean/lastp_r_p_p.md (scalar index)
///
/// LASTA (scalar): Extract element AFTER last active element to X[d] (zero-extended)
/// LASTA (SIMD&FP): Extract element AFTER last active element to V[d]
/// LASTB (scalar): Extract LAST active element to X[d] (zero-extended)
/// LASTB (SIMD&FP): Extract LAST active element to V[d]
/// LASTP: Find index of last true predicate element, place in X[d]
///
/// isBefore parameter: TRUE for LASTB (last active), FALSE for LASTA (after last)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn lasta_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 8, 16, 32, 64
    let rsize = BitWidth::W64; // rsize = 32 if esize < 64 else 64

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn

    // LASTA: isBefore = FALSE (element after last active)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::LastA_Scalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0), // X[d] - general-purpose register
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LASTA".to_string(),
            encoding: Some("Extract element after last active to general-purpose register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lasta_r_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn lasta_simd_fp_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::LastA_Scalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0), // V[d] - SIMD&FP scalar register
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LASTA".to_string(),
            encoding: Some("Extract element after last active to SIMD&FP scalar register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lasta_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn lastb_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;
    let rsize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn

    // LASTB: isBefore = TRUE (last active element)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::LastB_Scalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0), // X[d] - general-purpose register
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LASTB".to_string(),
            encoding: Some("Extract last active element to general-purpose register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lastb_r_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn lastb_simd_fp_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::LastB_Scalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0), // V[d] - SIMD&FP scalar register
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LASTB".to_string(),
            encoding: Some("Extract last active element to SIMD&FP scalar register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lastb_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn lastp_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 8, 16, 32, 64

    let predicate = Expr::ReadPredReg { reg: 0, pl }; // Pg
    let operand = Expr::ReadPredReg { reg: 1, pl }; // Pn

    // LASTP: find index of last true predicate element
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::LastP,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0), // X[d] - 64-bit result
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LASTP".to_string(),
            encoding: Some("Scalar index of last true predicate element".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lastp_r_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lasta_scalar_sve_validates() {
    let instruction = lasta_scalar_sve_instruction();
    validate_instruction(&instruction).expect("LASTA scalar SVE should validate");
}

#[test]
fn lasta_scalar_sve_structure() {
    let instruction = lasta_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for LASTA scalar");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::LastA_Scalar);
    assert!(predicate.is_some());
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn lasta_simd_fp_sve_validates() {
    let instruction = lasta_simd_fp_sve_instruction();
    validate_instruction(&instruction).expect("LASTA SIMD&FP SVE should validate");
}

#[test]
fn lasta_simd_fp_sve_structure() {
    let instruction = lasta_simd_fp_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for LASTA SIMD&FP");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::LastA_Scalar);
    assert!(predicate.is_some());
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn lastb_scalar_sve_validates() {
    let instruction = lastb_scalar_sve_instruction();
    validate_instruction(&instruction).expect("LASTB scalar SVE should validate");
}

#[test]
fn lastb_scalar_sve_structure() {
    let instruction = lastb_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for LASTB scalar");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::LastB_Scalar);
    assert!(predicate.is_some());
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn lastb_simd_fp_sve_validates() {
    let instruction = lastb_simd_fp_sve_instruction();
    validate_instruction(&instruction).expect("LASTB SIMD&FP SVE should validate");
}

#[test]
fn lastb_simd_fp_sve_structure() {
    let instruction = lastb_simd_fp_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for LASTB SIMD&FP");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::LastB_Scalar);
    assert!(predicate.is_some());
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn lastp_sve_validates() {
    let instruction = lastp_sve_instruction();
    validate_instruction(&instruction).expect("LASTP SVE should validate");
}

#[test]
fn lastp_sve_structure() {
    let instruction = lastp_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for LASTP");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::LastP);
    assert!(predicate.is_some());
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadPredReg { reg: 1, pl: BitWidth::W32 });
}