/// LD1SB, LD1SH, LD1SW (SVE) - Signed loads (non-broadcast)
/// Stanford sources:
///   - ld1sb_z_p_bz.md (vector index, gather)
///   - ld1sb_z_p_br.md (scalar index, contiguous)
///   - ld1sb_z_p_bi.md (immediate index, contiguous)
///   - ld1sb_z_p_ai.md (vector base + immediate, gather)
///   - ld1sh_z_p_* (halfword variants)
///   - ld1sw_z_p_* (word variants)
///
/// LD1SB: signed byte (8-bit) load with sign extension
/// LD1SH: signed halfword (16-bit) load with sign extension
/// LD1SW: signed word (32-bit) load with sign extension
///
/// Variants:
/// 1. Contiguous load with scalar base + scalar index (X[m])
/// 2. Contiguous load with scalar base + immediate index (imm4, SInt)
/// 3. Gather load with vector base (Zn) + vector index (Zm)
/// 4. Gather load with vector base (Zn) + immediate index (imm5)
/// All are predicated with zeroing (inactive = 0)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1sb_sve_gather_vector_index_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Vector element size (64-bit)
    let msize = BitWidth::W8;  // Memory access size (8-bit signed byte)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }; // X[n]
    let index = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[m] - vector index

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false, // Signed extension
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1SB".to_string(),
            encoding: Some("Gather load signed bytes to vector (vector index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sb_z_p_bz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1sb_sve_contiguous_scalar_index_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }; // Scalar index X[m]

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
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
            mnemonic: "LD1SB".to_string(),
            encoding: Some("Contiguous load signed bytes to vector (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sb_z_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1sb_sve_contiguous_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let imm_index = Expr::Const { value: 3i32 as u128, ty: BitWidth::W4 }; // SInt(imm4) = 3

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(predicate),
        base: Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
        index: Box::new(imm_index),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
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
            mnemonic: "LD1SB".to_string(),
            encoding: Some("Contiguous load signed bytes to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sb_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1sb_sve_gather_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadScalableVecReg { reg: 1, vl }; // Vector base Zn
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Immediate index: imm5 = 4 (range 0-31)
    let imm_index = Expr::Const { value: 4, ty: BitWidth::new(5).unwrap() };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(imm_index),
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
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1SB".to_string(),
            encoding: Some("Gather load signed bytes to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sb_z_p_ai.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// LD1SH variants (halfword, 16-bit, msize=16)

fn ld1sh_sve_gather_vector_index_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W16; // Halfword (16-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false, // Signed extension
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1SH".to_string(),
            encoding: Some("Gather load signed halfwords to vector (vector index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sh_z_p_bz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld1sw_sve_gather_vector_index_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W32; // Word (32-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false, // Signed extension
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1SW".to_string(),
            encoding: Some("Gather load signed words to vector (vector index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sw_z_p_bz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1sb_sve_gather_vector_index_validates() {
    let instruction = ld1sb_sve_gather_vector_index_instruction();
    validate_instruction(&instruction).expect("LD1SB SVE gather vector index should validate");
}

#[test]
fn ld1sb_sve_gather_vector_index_structure() {
    let instruction = ld1sb_sve_gather_vector_index_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, data_unsigned, .. } = value else {
        panic!("expected SVEGatherScatter for LD1SB gather vector index");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::GatherLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*contiguous, false);
    assert_eq!(*data_unsigned, false); // Signed extension
    matches!(base.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 });
    matches!(index.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn ld1sb_sve_contiguous_scalar_index_validates() {
    let instruction = ld1sb_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD1SB contiguous scalar index should validate");
}

#[test]
fn ld1sb_sve_contiguous_scalar_index_structure() {
    let instruction = ld1sb_sve_contiguous_scalar_index_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, contiguous, data_unsigned, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1SB contiguous");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
    assert_eq!(*contiguous, true);
    assert_eq!(*data_unsigned, false); // Signed extension
    assert_eq!(*msize, BitWidth::W8);
}

#[test]
fn ld1sb_sve_contiguous_imm_validates() {
    let instruction = ld1sb_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD1SB contiguous immediate should validate");
}

#[test]
fn ld1sb_sve_gather_imm_validates() {
    let instruction = ld1sb_sve_gather_imm_instruction();
    validate_instruction(&instruction).expect("LD1SB gather immediate should validate");
}

#[test]
fn ld1sh_sve_gather_vector_index_validates() {
    let instruction = ld1sh_sve_gather_vector_index_instruction();
    validate_instruction(&instruction).expect("LD1SH gather vector index should validate");
}

#[test]
fn ld1sh_sve_gather_vector_index_structure() {
    let instruction = ld1sh_sve_gather_vector_index_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, .. } = value else {
        panic!("expected SVEGatherScatter for LD1SH");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::GatherLoad);
    assert_eq!(*msize, BitWidth::W16); // Halfword
    assert_eq!(*data_unsigned, false);
}

#[test]
fn ld1sw_sve_gather_vector_index_validates() {
    let instruction = ld1sw_sve_gather_vector_index_instruction();
    validate_instruction(&instruction).expect("LD1SW gather vector index should validate");
}

#[test]
fn ld1sw_sve_gather_vector_index_structure() {
    let instruction = ld1sw_sve_gather_vector_index_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, .. } = value else {
        panic!("expected SVEGatherScatter for LD1SW");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::GatherLoad);
    assert_eq!(*msize, BitWidth::W32); // Word
    assert_eq!(*data_unsigned, false);
}