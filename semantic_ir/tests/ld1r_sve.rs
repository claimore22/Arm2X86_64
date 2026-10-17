/// LD1RB, LD1RH, LD1RW, LD1RD (SVE predicated) - Load and broadcast to vector
/// Stanford sources: arm64_md_clean/ld1rb_z_p_bi.md, ld1rh_z_p_bi.md, ld1rw_z_p_bi.md, ld1rd_z_p_bi.md
///
/// LD1RB: Load and broadcast unsigned byte (8-bit) to all active elements
/// LD1RH: Load and broadcast unsigned halfword (16-bit) to all active elements
/// LD1RW: Load and broadcast unsigned word (32-bit) to all active elements
/// LD1RD: Load and broadcast doubleword (64-bit) to all active elements
///
/// All variants:
/// - Load single element from memory at base + offset
/// - Broadcast to all active elements of destination vector
/// - Inactive elements set to zero
/// - Scalar base (X[n] or SP) with immediate offset
/// - Predicated with zeroing (inactive elements = 0)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1rb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Vector element size
    let msize = BitWidth::W8;  // Memory access size (8-bit byte)

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    // Base address: X[0] (general-purpose register)
    let base = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    // Immediate offset: imm6 = 8 (range 0-63 for bytes)
    let imm_offset = Expr::Const {
        value: 8u128,
        ty: BitWidth::new(6).unwrap(), // 6-bit immediate
    };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // LD1RB: contiguous load with broadcast
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
        data_unsigned: true,
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
            mnemonic: "LD1RB".to_string(),
            encoding: Some("Load and broadcast unsigned byte to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rb_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1rh_sve_instruction() -> Instruction {
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
        data_unsigned: true,
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
            mnemonic: "LD1RH".to_string(),
            encoding: Some("Load and broadcast unsigned halfword to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rh_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1rw_sve_instruction() -> Instruction {
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
        data_unsigned: true,
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
            mnemonic: "LD1RW".to_string(),
            encoding: Some("Load and broadcast unsigned word to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rw_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1rd_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W64; // Doubleword (64-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let imm_offset = Expr::Const { value: 56u128, ty: BitWidth::new(6).unwrap() }; // Range 0-63 for doublewords (multiple of 8)
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
        data_unsigned: true,
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
            mnemonic: "LD1RD".to_string(),
            encoding: Some("Load and broadcast doubleword to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1rd_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1rb_sve_validates() {
    let instruction = ld1rb_sve_instruction();
    validate_instruction(&instruction).expect("LD1RB SVE should validate");
}

#[test]
fn ld1rb_sve_structure() {
    let instruction = ld1rb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RB");
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
fn ld1rh_sve_validates() {
    let instruction = ld1rh_sve_instruction();
    validate_instruction(&instruction).expect("LD1RH SVE should validate");
}

#[test]
fn ld1rh_sve_structure() {
    let instruction = ld1rh_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RH");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad);
    assert_eq!(*msize, BitWidth::W16);
}

#[test]
fn ld1rw_sve_validates() {
    let instruction = ld1rw_sve_instruction();
    validate_instruction(&instruction).expect("LD1RW SVE should validate");
}

#[test]
fn ld1rw_sve_structure() {
    let instruction = ld1rw_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RW");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad);
    assert_eq!(*msize, BitWidth::W32);
}

#[test]
fn ld1rd_sve_validates() {
    let instruction = ld1rd_sve_instruction();
    validate_instruction(&instruction).expect("LD1RD SVE should validate");
}

#[test]
fn ld1rd_sve_structure() {
    let instruction = ld1rd_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1RD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ReplicateLoad);
    assert_eq!(*msize, BitWidth::W64);
}