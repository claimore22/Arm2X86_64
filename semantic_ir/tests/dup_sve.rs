/// DUP (immediate) - Broadcast signed immediate to vector elements (unpredicated)
/// Stanford source: arm64_md_clean/dup_z_i.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size);
///   let d : integer = UInt(Zd);
///   var imm : integer = SInt(imm8);
///   if sh == '1' then imm = imm << 8; end;
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let result : bits(VL) = Replicate(imm[esize-1:0]);
///   Z[VL](d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn dup_imm_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let d = 0u8;

    // imm8 = 42, sh = 0 (no shift)
    let imm = Expr::Const { value: 42i8 as u128, ty: BitWidth::W32 };

    // Replicate immediate to fill the vector
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(imm)],
        vl,
        esize: Some(esize),
        imm: Some(0), // sh = 0
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: d,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUP".to_string(),
            encoding: Some("Broadcast signed immediate to vector elements (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dup_z_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dup_imm_sve_validates() {
    let instruction = dup_imm_sve_instruction();
    validate_instruction(&instruction).expect("DUP (immediate) SVE should validate");
}

#[test]
fn dup_imm_sve_uses_special_op() {
    let instruction = dup_imm_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for DUP immediate");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert!(matches!(operands[0].as_ref(), Expr::Const { value: 42, ty: BitWidth::W32 }));
}

// DUP (indexed) - Broadcast indexed element to vector (unpredicated)
// Stanford source: arm64_md_clean/dup_z_zi.md
fn dup_indexed_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let n = 1u8;
    let d = 0u8;
    let index = 2u8;

    let operand1 = Expr::ReadScalableVecReg { reg: n, vl: BitWidth::W256 };

    // DUP (indexed): extract element at index and replicate
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(operand1)],
        vl: BitWidth::W256,
        esize: Some(BitWidth::W32),
        imm: Some(index as u16),
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: d,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUP".to_string(),
            encoding: Some("Broadcast indexed element to vector (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dup_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dup_indexed_sve_validates() {
    let instruction = dup_indexed_sve_instruction();
    validate_instruction(&instruction).expect("DUP (indexed) SVE should validate");
}

#[test]
fn dup_indexed_sve_uses_special_op() {
    let instruction = dup_indexed_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, operands, imm, .. } = value else {
        panic!("expected SVESpecial for DUP indexed");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert!(matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert_eq!(*imm, Some(2));
}

// DUPM - Broadcast logical bitmask immediate to vector (unpredicated)
// Stanford source: arm64_md_clean/dupm_z_i.md
fn dupm_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let d = 0u8;

    // DUPM: broadcasts a 64-bit logical bitmask immediate
    let imm = Expr::Const { value: 0xAAAAAAAAAAAAAAAAu128, ty: BitWidth::W64 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(imm)],
        vl,
        esize: Some(BitWidth::W64),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: d,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUPM".to_string(),
            encoding: Some("Broadcast logical bitmask immediate to vector (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dupm_z_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dupm_sve_validates() {
    let instruction = dupm_sve_instruction();
    validate_instruction(&instruction).expect("DUPM SVE should validate");
}

#[test]
fn dupm_sve_uses_special_op() {
    let instruction = dupm_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, operands, esize, .. } = value else {
        panic!("expected SVESpecial for DUPM");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(*esize, Some(BitWidth::W64));
    assert!(matches!(operands[0].as_ref(), Expr::Const { value: 0xAAAAAAAAAAAAAAAA, ty: BitWidth::W64 }));
}

// DUPQ - Broadcast indexed element within each quadword vector segment (unpredicated)
// Stanford source: arm64_md_clean/dupq_z_zi.md
fn dupq_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let n = 1u8;
    let d = 0u8;
    let index = 2u8; // element index within 128-bit segment

    let operand1 = Expr::ReadScalableVecReg { reg: n, vl: BitWidth::W256 };

    // DUPQ: extract element at index within each 128-bit segment and replicate within segment
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(operand1)],
        vl: BitWidth::W256,
        esize: Some(BitWidth::W32),
        imm: Some(index as u16),
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: d,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUPQ".to_string(),
            encoding: Some("Broadcast indexed element within each quadword vector segment (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dupq_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dupq_sve_validates() {
    let instruction = dupq_sve_instruction();
    validate_instruction(&instruction).expect("DUPQ SVE should validate");
}

#[test]
fn dupq_sve_uses_special_op() {
    let instruction = dupq_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, imm, .. } = value else {
        panic!("expected SVESpecial for DUPQ");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(*imm, Some(2));
}

// DUP (scalar) - Broadcast general-purpose register to vector elements (unpredicated)
// Stanford source: arm64_md_clean/dup_z_r.md
fn dup_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let n = 1u8; // Rn (not SP)
    let d = 0u8;

    // Source register X1
    let operand = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 };

    // Broadcast scalar to vector
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 })],
        vl,
        esize: Some(BitWidth::W64), // Source is 64-bit
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUP".to_string(),
            encoding: Some("Broadcast general-purpose register to vector elements (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dup_z_r.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dup_scalar_sve_validates() {
    let instruction = dup_scalar_sve_instruction();
    validate_instruction(&instruction).expect("DUP (scalar) SVE should validate");
}

#[test]
fn dup_scalar_sve_uses_special_op() {
    let instruction = dup_scalar_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, operands, esize, .. } = value else {
        panic!("expected SVESpecial for DUP scalar");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(*esize, Some(BitWidth::W64));
    assert!(matches!(operands[0].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }));
}