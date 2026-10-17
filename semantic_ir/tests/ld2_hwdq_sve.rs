/// LD2H, LD2W, LD2D, LD2Q (SVE) - Contiguous load two-element structures to two vectors
/// Stanford sources:
///   - ld2h_z_p_bi.md, ld2h_z_p_br.md (halfwords)
///   - ld2w_z_p_bi.md, ld2w_z_p_br.md (words)
///   - ld2d_z_p_bi.md, ld2d_z_p_br.md (doublewords)
///   - ld2q_z_p_bi.md, ld2q_z_p_br.md (quadwords)
///
/// LD2: Load 2-element structures to two consecutive vector registers (nreg=2)
/// Each structure = 2 elements of size esize (e.g., 2 halfwords, 2 words, etc.)
/// Variants: immediate index (imm4, SInt, range -16 to 14) and scalar index (X[m])
/// Both variants: predicated with zeroing (inactive elements = 0)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn create_ld2_contiguous_imm_instruction(
    mnemonic: &str,
    esize: BitWidth,
    msize: BitWidth,
    offset: i32,
) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let nreg = 2;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let imm_offset = Expr::Const { value: offset as u128, ty: BitWidth::W4 };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 };
    let destination2 = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: Some(Box::new(predicate)),
        operands: vec![
            Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
            Box::new(Expr::Const { value: offset as u128, ty: BitWidth::W4 }),
        ],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![
        Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl: BitWidth::W256 },
        Stmt::WriteScalableVecReg { reg: 1, value: result, vl: BitWidth::W256 },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Contiguous load two-{} structures to two vectors (immediate index)", 
                match msize { BitWidth::W16 => "halfword", BitWidth::W32 => "word", BitWidth::W64 => "doubleword", _ => "quadword" })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: format!("ld2{}_z_p_bi.md", match msize { BitWidth::W16 => "h", BitWidth::W32 => "w", BitWidth::W64 => "d", _ => "q" }),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn create_ld2_contiguous_scalar_index_instruction(
    mnemonic: &str,
    esize: BitWidth,
    msize: BitWidth,
) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let msize_val = msize;
    let nreg = 2;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 };
    let destination2 = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        base: Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
        index: Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        vl: BitWidth::W256,
        esize,
        msize: msize_val,
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
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Contiguous load two-{} structures to two vectors (scalar index)", 
                match esize { BitWidth::W16 => "halfword", BitWidth::W32 => "word", BitWidth::W64 => "doubleword", _ => "quadword" })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: format!("ld2{}_z_p_br.md", match esize { BitWidth::W16 => "h", BitWidth::W32 => "w", BitWidth::W64 => "d", _ => "q" }),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ld2h_sve_contiguous_imm_instruction() -> Instruction {
    create_ld2_contiguous_imm_instruction("LD2H", BitWidth::W16, BitWidth::W16, 2)
}

fn ld2w_sve_contiguous_imm_instruction() -> Instruction {
    create_ld2_contiguous_imm_instruction("LD2W", BitWidth::W32, BitWidth::W32, 2)
}

fn ld2d_sve_contiguous_imm_instruction() -> Instruction {
    create_ld2_contiguous_imm_instruction("LD2D", BitWidth::W64, BitWidth::W64, 2)
}

fn ld2q_sve_contiguous_imm_instruction() -> Instruction {
    create_ld2_contiguous_imm_instruction("LD2Q", BitWidth::W128, BitWidth::W128, 2)
}

fn ld2h_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld2_contiguous_scalar_index_instruction("LD2H", BitWidth::W16, BitWidth::W16)
}

fn ld2w_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld2_contiguous_scalar_index_instruction("LD2W", BitWidth::W32, BitWidth::W32)
}

fn ld2d_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld2_contiguous_scalar_index_instruction("LD2D", BitWidth::W64, BitWidth::W64)
}

fn ld2q_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld2_contiguous_scalar_index_instruction("LD2Q", BitWidth::W128, BitWidth::W128)
}

#[test]
fn ld2h_sve_contiguous_imm_validates() {
    let instruction = ld2h_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD2H contiguous immediate should validate");
}

#[test]
fn ld2h_sve_contiguous_imm_structure() {
    let instruction = ld2h_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD2H"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W16));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld2w_sve_contiguous_imm_validates() {
    let instruction = ld2w_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD2W contiguous immediate should validate");
}

#[test]
fn ld2w_sve_contiguous_imm_structure() {
    let instruction = ld2w_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD2W"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W32));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld2d_sve_contiguous_imm_validates() {
    let instruction = ld2d_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD2D contiguous immediate should validate");
}

#[test]
fn ld2d_sve_contiguous_imm_structure() {
    let instruction = ld2d_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD2D"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W64));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld2q_sve_contiguous_imm_validates() {
    let instruction = ld2q_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD2Q contiguous immediate should validate");
}

#[test]
fn ld2q_sve_contiguous_imm_structure() {
    let instruction = ld2q_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD2Q"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W128));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld2h_sve_contiguous_scalar_index_validates() {
    let instruction = ld2h_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD2H contiguous scalar index should validate");
}

#[test]
fn ld2h_sve_contiguous_scalar_index_structure() {
    let instruction = ld2h_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD2H scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W16);
        assert_eq!(*msize, BitWidth::W16);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld2w_sve_contiguous_scalar_index_validates() {
    let instruction = ld2w_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD2W contiguous scalar index should validate");
}

#[test]
fn ld2w_sve_contiguous_scalar_index_structure() {
    let instruction = ld2w_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD2W scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W32);
        assert_eq!(*msize, BitWidth::W32);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld2d_sve_contiguous_scalar_index_validates() {
    let instruction = ld2d_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD2D contiguous scalar index should validate");
}

#[test]
fn ld2d_sve_contiguous_scalar_index_structure() {
    let instruction = ld2d_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD2D scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W64);
        assert_eq!(*msize, BitWidth::W64);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld2q_sve_contiguous_scalar_index_validates() {
    let instruction = ld2q_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD2Q contiguous scalar index should validate");
}

#[test]
fn ld2q_sve_contiguous_scalar_index_structure() {
    let instruction = ld2q_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 2);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD2Q scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W128);
        assert_eq!(*msize, BitWidth::W128);
        assert_eq!(*contiguous, true);
    }
}