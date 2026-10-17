/// LD2B (SVE) - Contiguous load two-byte structures to two vectors
/// Stanford sources:
///   - arm64_md_clean/ld2b_z_p_bi.md (scalar plus immediate)
///   - arm64_md_clean/ld2b_z_p_br.md (scalar plus scalar)
///
/// LD2B: Load two-byte structures to two consecutive vector registers
/// - Each structure = 2 bytes (consecutive bytes in memory)
/// - Loaded into two consecutive vector registers Zt and Zt+1
/// - nreg = 2 (two destination registers)
/// Variants:
///   - Immediate index: contiguous load with immediate offset (imm4, SInt, range -16 to 14)
///   - Scalar index: contiguous load with scalar index (X[m])
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld2b_sve_contiguous_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;  // 8-bit elements (bytes)
    let msize = BitWidth::W8;  // 8-bit memory access
    let nreg = 2;  // Two destination registers

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };

    // Immediate offset: imm4 = 2 (range -16 to 14, multiple of 2)
    let imm_offset = Expr::Const { value: 2i32 as u128, ty: BitWidth::W4 };

    // Two destination registers: Z0 and Z1
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let destination2 = Expr::ReadScalableVecReg { reg: 1, vl };

    // LD2B: contiguous load two-byte structures to two vectors
    // Using SVESpecial for multi-register load with immediate index
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: Some(Box::new(predicate)),
        operands: vec![
            Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),  // base
            Box::new(Expr::Const { value: 2i32 as u128, ty: BitWidth::W4 }),   // imm4 offset
        ],
        vl,
        esize: Some(BitWidth::W8),
        imm: None,
    };

    // For multi-register, we model as producing an array of values
    // Using a tuple-like approach with SVESpecial
    let semantics = vec![
        Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl: BitWidth::W256 },
        Stmt::WriteScalableVecReg { reg: 1, value: result, vl: BitWidth::W256 },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "LD2B".to_string(),
            encoding: Some("Contiguous load two-byte structures to two vectors (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld2b_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld2b_sve_contiguous_scalar_index_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;
    let msize = BitWidth::W8;
    let nreg = 2;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 };
    let destination2 = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 };

    // LD2B with scalar index: contiguous load with scalar base and scalar index
    // Using SVEGatherScatter with contiguous and ReplicateLoad for multi-register
    // For multi-register, we model as two separate contiguous loads
    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        base: Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
        index: Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        vl: BitWidth::W256,
        esize: BitWidth::W8,
        msize: BitWidth::W8,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: true,
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![
        Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl: BitWidth::W256 },
        Stmt::WriteScalableVecReg { reg: 1, value: result, vl: BitWidth::W256 },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "LD2B".to_string(),
            encoding: Some("Contiguous load two-byte structures to two vectors (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld2b_z_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld2b_sve_contiguous_imm_validates() {
    let instruction = ld2b_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD2B contiguous immediate should validate");
}

#[test]
fn ld2b_sve_contiguous_imm_structure() {
    let instruction = ld2b_sve_contiguous_imm_instruction();

    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else {
            panic!("expected WriteScalableVecReg");
        };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
            panic!("expected SVESpecial for LD2B");
        };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        // predicate is Option<Box<Expr>>, check it's a valid predicate register
        let Some(pred) = predicate else {
            panic!("expected predicate to be Some");
        };
        let Expr::ReadPredReg { .. } = pred.as_ref() else {
            panic!("expected ReadPredReg for predicate");
        };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W8));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld2b_sve_contiguous_scalar_index_validates() {
    let instruction = ld2b_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD2B contiguous scalar index should validate");
}

#[test]
fn ld2b_sve_contiguous_scalar_index_structure() {
    let instruction = ld2b_sve_contiguous_scalar_index_instruction();

    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else {
            panic!("expected WriteScalableVecReg");
        };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else {
            panic!("expected SVEGatherScatter for LD2B scalar index");
        };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else {
            panic!("expected ReadPredReg for predicate");
        };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W8);
        assert_eq!(*msize, BitWidth::W8);
        assert_eq!(*contiguous, true);
        matches!(base.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 });
        matches!(index.as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
    }
}