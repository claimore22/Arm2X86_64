/// LD3B, LD3H, LD3W, LD3D, LD3Q, LD4B, LD4H, LD4W, LD4D, LD4Q (SVE)
/// Stanford sources: arm64_md_clean/ld3*_z_p_b*.md, ld4*_z_p_b*.md
///
/// LD3: Contiguous load three-element structures to three vectors (nreg=3)
/// LD4: Contiguous load four-element structures to four vectors (nreg=4)
///
/// Each structure = nreg elements of size esize (e.g., 3 halfwords, 4 words, etc.)
/// Variants: immediate index (imm4, SInt, range -24 to 21 for LD3, -32 to 28 for LD4)
///            scalar index (X[m])
/// Both variants: predicated with zeroing (inactive elements = 0)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn create_ld3_ld4_contiguous_imm_instruction(
    mnemonic: &str,
    nreg: u8,
    esize: BitWidth,
    msize: BitWidth,
    offset: i32,
) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let imm_offset = Expr::Const { value: offset as u128, ty: BitWidth::W4 };

    // nreg destination registers: Z0, Z1, Z2 (and Z3 for LD4)
    let mut destinations = Vec::new();
    for i in 0..nreg {
        destinations.push(Expr::ReadScalableVecReg { reg: i, vl: BitWidth::W256 });
    }

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Index,
        predicate: Some(Box::new(predicate)),
        operands: vec![
            Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
            Box::new(Expr::Const { value: offset as u128, ty: BitWidth::W4 }),
        ],
        vl: BitWidth::W256,
        esize: Some(esize),
        imm: None,
    };

    let mut semantics = Vec::new();
    for i in 0..nreg {
        semantics.push(Stmt::WriteScalableVecReg {
            reg: i,
            value: result.clone(),
            vl: BitWidth::W256,
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: format!("LD{}", &mnemonic[2..]), // Remove "LD" prefix to get register count
            encoding: Some(format!("Contiguous load {}-{} structures to {} vectors (immediate index)",
                nreg, nreg,
                match msize { 
                    BitWidth::W8 => "byte", 
                    BitWidth::W16 => "halfword", 
                    BitWidth::W32 => "word", 
                    BitWidth::W64 => "doubleword", 
                    _ => "quadword" })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: format!("ld{}{}_z_p_bi.md", &mnemonic[2..], match msize { BitWidth::W8 => "b", BitWidth::W16 => "h", BitWidth::W32 => "w", BitWidth::W64 => "d", _ => "q" }),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn create_ld3_ld4_contiguous_scalar_index_instruction(
    mnemonic: &str,
    nreg: u8,
    esize: BitWidth,
    msize: BitWidth,
) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let msize_val = msize;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        base: Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
        index: Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        vl: BitWidth::W256,
        esize,
        msize: msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: true,
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: true,
        tag_checked: true,
    };

    let mut semantics = Vec::new();
    for i in 0..nreg {
        semantics.push(Stmt::WriteScalableVecReg {
            reg: i,
            value: result.clone(),
            vl: BitWidth::W256,
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: format!("LD{}", &mnemonic[2..]),
            encoding: Some(format!("Contiguous load {}-{} structures to {} vectors (scalar index)",
                nreg, nreg,
                match msize { 
                    BitWidth::W8 => "byte", 
                    BitWidth::W16 => "halfword", 
                    BitWidth::W32 => "word", 
                    BitWidth::W64 => "doubleword", 
                    _ => "quadword" }),
            ),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics: (0..nreg).map(|i| {
            Stmt::WriteScalableVecReg {
                reg: i,
                value: Expr::SVEGatherScatter {
                    op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
                    predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
                    base: Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
                    index: Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }),
                    destination: Box::new(Expr::ReadScalableVecReg { reg: i, vl: BitWidth::W256 }),
                    vl: BitWidth::W256,
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
                },
                vl: BitWidth::W256,
            }
        }).collect(),
        provenance: Provenance {
            source_path: format!("ld{}{}_z_p_br.md", &mnemonic[2..], match msize { BitWidth::W8 => "b", BitWidth::W16 => "h", BitWidth::W32 => "w", BitWidth::W64 => "d", _ => "q" }),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// LD3 instructions
fn ld3b_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD3B", 3, BitWidth::W8, BitWidth::W8, 3)
}

fn ld3h_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD3H", 3, BitWidth::W16, BitWidth::W16, 3)
}

fn ld3w_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD3W", 3, BitWidth::W32, BitWidth::W32, 3)
}

fn ld3d_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD3D", 3, BitWidth::W64, BitWidth::W64, 3)
}

fn ld3q_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD3Q", 3, BitWidth::W128, BitWidth::W128, 3)
}

fn ld3b_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD3B", 3, BitWidth::W8, BitWidth::W8)
}

fn ld3h_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD3H", 3, BitWidth::W16, BitWidth::W16)
}

fn ld3w_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD3W", 3, BitWidth::W32, BitWidth::W32)
}

fn ld3d_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD3D", 3, BitWidth::W64, BitWidth::W64)
}

fn ld3q_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD3Q", 3, BitWidth::W128, BitWidth::W128)
}

// LD4 instructions
fn ld4b_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD4B", 4, BitWidth::W8, BitWidth::W8, 4)
}

fn ld4h_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD4H", 4, BitWidth::W16, BitWidth::W16, 4)
}

fn ld4w_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD4W", 4, BitWidth::W32, BitWidth::W32, 4)
}

fn ld4d_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD4D", 4, BitWidth::W64, BitWidth::W64, 4)
}

fn ld4q_sve_contiguous_imm_instruction() -> Instruction {
    create_ld3_ld4_contiguous_imm_instruction("LD4Q", 4, BitWidth::W128, BitWidth::W128, 4)
}

fn ld4b_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD4B", 4, BitWidth::W8, BitWidth::W8)
}

fn ld4h_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD4H", 4, BitWidth::W16, BitWidth::W16)
}

fn ld4w_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD4W", 4, BitWidth::W32, BitWidth::W32)
}

fn ld4d_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD4D", 4, BitWidth::W64, BitWidth::W64)
}

fn ld4q_sve_contiguous_scalar_index_instruction() -> Instruction {
    create_ld3_ld4_contiguous_scalar_index_instruction("LD4Q", 4, BitWidth::W128, BitWidth::W128)
}

// Tests for LD3
#[test]
fn ld3b_sve_contiguous_imm_validates() {
    let instruction = ld3b_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD3B contiguous immediate should validate");
}

#[test]
fn ld3b_sve_contiguous_imm_structure() {
    let instruction = ld3b_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD3B"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W8));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld3h_sve_contiguous_imm_validates() {
    let instruction = ld3h_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD3H contiguous immediate should validate");
}

#[test]
fn ld3h_sve_contiguous_imm_structure() {
    let instruction = ld3h_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD3H"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W16));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld3w_sve_contiguous_imm_validates() {
    let instruction = ld3w_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD3W contiguous immediate should validate");
}

#[test]
fn ld3w_sve_contiguous_imm_structure() {
    let instruction = ld3w_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD3W"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W32));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld3d_sve_contiguous_imm_validates() {
    let instruction = ld3d_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD3D contiguous immediate should validate");
}

#[test]
fn ld3d_sve_contiguous_imm_structure() {
    let instruction = ld3d_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD3D"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W64));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld3q_sve_contiguous_imm_validates() {
    let instruction = ld3q_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD3Q contiguous immediate should validate");
}

#[test]
fn ld3q_sve_contiguous_imm_structure() {
    let instruction = ld3q_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD3Q"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W128));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld3b_sve_contiguous_scalar_index_validates() {
    let instruction = ld3b_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD3B contiguous scalar index should validate");
}

#[test]
fn ld3b_sve_contiguous_scalar_index_structure() {
    let instruction = ld3b_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD3B scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W8);
        assert_eq!(*msize, BitWidth::W8);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld3h_sve_contiguous_scalar_index_validates() {
    let instruction = ld3h_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD3H contiguous scalar index should validate");
}

#[test]
fn ld3h_sve_contiguous_scalar_index_structure() {
    let instruction = ld3h_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD3H scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W16);
        assert_eq!(*msize, BitWidth::W16);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld3w_sve_contiguous_scalar_index_validates() {
    let instruction = ld3w_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD3W contiguous scalar index should validate");
}

#[test]
fn ld3w_sve_contiguous_scalar_index_structure() {
    let instruction = ld3w_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD3W scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W32);
        assert_eq!(*msize, BitWidth::W32);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld3d_sve_contiguous_scalar_index_validates() {
    let instruction = ld3d_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD3D contiguous scalar index should validate");
}

#[test]
fn ld3d_sve_contiguous_scalar_index_structure() {
    let instruction = ld3d_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD3D scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W64);
        assert_eq!(*msize, BitWidth::W64);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld3q_sve_contiguous_scalar_index_validates() {
    let instruction = ld3q_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD3Q contiguous scalar index should validate");
}

#[test]
fn ld3q_sve_contiguous_scalar_index_structure() {
    let instruction = ld3q_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 3);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD3Q scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W128);
        assert_eq!(*msize, BitWidth::W128);
        assert_eq!(*contiguous, true);
    }
}

// LD4 Tests
#[test]
fn ld4b_sve_contiguous_imm_validates() {
    let instruction = ld4b_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD4B contiguous immediate should validate");
}

#[test]
fn ld4b_sve_contiguous_imm_structure() {
    let instruction = ld4b_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD4B"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W8));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld4h_sve_contiguous_imm_validates() {
    let instruction = ld4h_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD4H contiguous immediate should validate");
}

#[test]
fn ld4h_sve_contiguous_imm_structure() {
    let instruction = ld4h_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD4H"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W16));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld4w_sve_contiguous_imm_validates() {
    let instruction = ld4w_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD4W contiguous immediate should validate");
}

#[test]
fn ld4w_sve_contiguous_imm_structure() {
    let instruction = ld4w_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD4W"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W32));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld4d_sve_contiguous_imm_validates() {
    let instruction = ld4d_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD4D contiguous immediate should validate");
}

#[test]
fn ld4d_sve_contiguous_imm_structure() {
    let instruction = ld4d_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD4D"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W64));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld4q_sve_contiguous_imm_validates() {
    let instruction = ld4q_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD4Q contiguous immediate should validate");
}

#[test]
fn ld4q_sve_contiguous_imm_structure() {
    let instruction = ld4q_sve_contiguous_imm_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else { panic!("expected SVESpecial for LD4Q"); };
        assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Index);
        let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
        let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, Some(BitWidth::W128));
        assert_eq!(operands.len(), 2);
    }
}

#[test]
fn ld4b_sve_contiguous_scalar_index_validates() {
    let instruction = ld4b_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD4B contiguous scalar index should validate");
}

#[test]
fn ld4b_sve_contiguous_scalar_index_structure() {
    let instruction = ld4b_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD4B scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W8);
        assert_eq!(*msize, BitWidth::W8);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld4h_sve_contiguous_scalar_index_validates() {
    let instruction = ld4h_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD4H contiguous scalar index should validate");
}

#[test]
fn ld4h_sve_contiguous_scalar_index_structure() {
    let instruction = ld4h_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD4H scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W16);
        assert_eq!(*msize, BitWidth::W16);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld4w_sve_contiguous_scalar_index_validates() {
    let instruction = ld4w_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD4W contiguous scalar index should validate");
}

#[test]
fn ld4w_sve_contiguous_scalar_index_structure() {
    let instruction = ld4w_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD4W scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W32);
        assert_eq!(*msize, BitWidth::W32);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld4d_sve_contiguous_scalar_index_validates() {
    let instruction = ld4d_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD4D contiguous scalar index should validate");
}

#[test]
fn ld4d_sve_contiguous_scalar_index_structure() {
    let instruction = ld4d_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD4D scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W64);
        assert_eq!(*msize, BitWidth::W64);
        assert_eq!(*contiguous, true);
    }
}

#[test]
fn ld4q_sve_contiguous_scalar_index_validates() {
    let instruction = ld4q_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD4Q contiguous scalar index should validate");
}

#[test]
fn ld4q_sve_contiguous_scalar_index_structure() {
    let instruction = ld4q_sve_contiguous_scalar_index_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    for stmt in &instruction.semantics {
        let Stmt::WriteScalableVecReg { value, .. } = stmt else { panic!("expected WriteScalableVecReg"); };
        let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LD4Q scalar index"); };
        assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
        let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
        assert_eq!(*vl, BitWidth::W256);
        assert_eq!(*esize, BitWidth::W128);
        assert_eq!(*msize, BitWidth::W128);
        assert_eq!(*contiguous, true);
    }
}