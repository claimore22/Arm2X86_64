/// LD1RSB, LD1RSH, LD1RSW (SVE predicated) - Load and broadcast signed to vector
/// Stanford sources:
///   - arm64_md_clean/ld1rsb_z_p_bi.md (signed byte)
///   - arm64_md_clean/ld1rsh_z_p_bi.md (signed halfword)
///   - arm64_md_clean/ld1rsw_z_p_bi.md (signed word)
///
/// All variants:
/// - Load single signed element from memory at base + offset
/// - Sign-extend and broadcast to all active elements of destination vector
/// - Inactive elements set to zero
/// - Scalar base (X[n] or SP) with immediate offset (imm6)
/// - Predicated with zeroing (inactive elements = 0)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1rsb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Vector element size
    let msize = BitWidth::W8;  // Memory access size (8-bit signed byte)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    // Immediate offset: imm6 = 8 (range 0-63 for bytes)
    let imm_offset = Expr::Const {
        value: 8u128,
        ty: BitWidth::new(6).unwrap(),
    };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // LD1RSB: contiguous load with broadcast, signed extension
    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(imm_offset),
        destination: Box::new(destination),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false, // Signed extension
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1RSB".to_string(),
            encoding: Some("Load and broadcast signed byte to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rsb_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1rsh_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W16; // Halfword (16-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let imm_offset = Expr::Const { value: 16u128, ty: BitWidth::new(6).unwrap() }; // Range 0-63 for halfwords (multiple of 2)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(imm_offset),
        destination: Box::new(destination),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false, // Signed extension
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1RSH".to_string(),
            encoding: Some("Load and broadcast signed halfword to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rsh_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1rsw_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W32; // Word (32-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let imm_offset = Expr::Const { value: 32u128, ty: BitWidth::new(6).unwrap() }; // Range 0-63 for words (multiple of 4)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(imm_offset),
        destination: Box::new(destination),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false, // Signed extension
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1RSW".to_string(),
            encoding: Some("Load and broadcast signed word to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rsw_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1rsb_sve_validates() {
    let instruction = ld1rsb_sve_instruction();
    validate_instruction(&instruction).expect("LD1RSB SVE should validate");
}

#[test]
fn ld1rsb_sve_structure() {
    let instruction = ld1rsb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RSB");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*contiguous, true);
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(base.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 });
    let expected_ty = BitWidth::new(6).unwrap();
    let Expr::Const { ty, .. } = index.as_ref() else { panic!("expected Const for index"); };
    assert_eq!(*ty, expected_ty);
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn ld1rsh_sve_validates() {
    let instruction = ld1rsh_sve_instruction();
    validate_instruction(&instruction).expect("LD1RSH SVE should validate");
}

#[test]
fn ld1rsh_sve_structure() {
    let instruction = ld1rsh_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RSH");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad);
    assert_eq!(*msize, BitWidth::W16);
}

#[test]
fn ld1rsw_sve_validates() {
    let instruction = ld1rsw_sve_instruction();
    validate_instruction(&instruction).expect("LD1RSW SVE should validate");
}

#[test]
fn ld1rsw_sve_structure() {
    let instruction = ld1rsw_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RSW");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad);
    assert_eq!(*msize, BitWidth::W32);
}