/// LDNF1B, LDNF1H, LDNF1W, LDNF1D, LDNF1SB, LDNF1SH, LDNF1SW (SVE) - Non-fault contiguous loads
/// Stanford sources: arm64_md_clean/ldnf1*_z_p_bi.md
///
/// These instructions perform contiguous loads with non-faulting behavior:
/// - If a memory access would fault, it returns zero instead of faulting
/// - Uses MemNF[] which returns (data, fault) instead of faulting
/// - faulted flag tracks if any access faulted
/// - contiguous = TRUE, non_fault = TRUE
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn create_ldnf1_contiguous_imm_instruction(
    mnemonic: &str,
    msize: BitWidth,
    data_unsigned: bool,
) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize_val = msize;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 };

    // Immediate offset: imm4 = 2 (range -8 to 7)
    let imm_offset = Expr::Const { value: 2i32 as u128, ty: BitWidth::W4 };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 };

    // LDNF1: contiguous load with non-faulting behavior
    // Using SVEGatherScatter with contiguous=true and non_fault=true
    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        base: Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 }),
        index: Box::new(Expr::Const { value: 2i32 as u128, ty: BitWidth::W4 }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        vl: BitWidth::W256,
        esize: BitWidth::W64,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned,
        first_fault: false,
        non_fault: true,
        non_temporal: false,
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result.clone(),
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Contiguous load non-fault {} to vector (immediate index)",
                match mnemonic {
                    "LDNF1B" => "unsigned bytes",
                    "LDNF1H" => "unsigned halfwords",
                    "LDNF1W" => "unsigned words",
                    "LDNF1D" => "doublewords",
                    "LDNF1SB" => "signed bytes",
                    "LDNF1SH" => "signed halfwords",
                    "LDNF1SW" => "signed words",
                    _ => "unknown",
                })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl: BitWidth::W256 }],
        provenance: Provenance {
            source_path: format!("ldnf1{}_z_p_bi.md", &mnemonic[5..]),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ldnf1b_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1B", BitWidth::W8, true)
}

fn ldnf1h_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1H", BitWidth::W16, true)
}

fn ldnf1w_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1W", BitWidth::W32, true)
}

fn ldnf1d_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1D", BitWidth::W64, true)
}

fn ldnf1sb_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1SB", BitWidth::W8, false)
}

fn ldnf1sh_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1SH", BitWidth::W16, false)
}

fn ldnf1sw_sve_contiguous_imm_instruction() -> Instruction {
    create_ldnf1_contiguous_imm_instruction("LDNF1SW", BitWidth::W32, false)
}

#[test]
fn ldnf1b_sve_contiguous_imm_validates() {
    let instruction = ldnf1b_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1B SVE contiguous immediate should validate");
}

#[test]
fn ldnf1b_sve_contiguous_imm_structure() {
    let instruction = ldnf1b_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, non_fault, .. } = value else { panic!("expected SVEGatherScatter for LDNF1B"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*contiguous, true);
    assert_eq!(*non_fault, true);
    let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
    matches!(base.as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 });
    matches!(index.as_ref(), Expr::Const { value: 2, ty: BitWidth::W4 });
}

#[test]
fn ldnf1h_sve_contiguous_imm_validates() {
    let instruction = ldnf1h_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1H SVE contiguous immediate should validate");
}

#[test]
fn ldnf1h_sve_contiguous_imm_structure() {
    let instruction = ldnf1h_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, non_fault, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LDNF1H"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*msize, BitWidth::W16);
    assert_eq!(*non_fault, true);
    assert_eq!(*contiguous, true);
}

#[test]
fn ldnf1w_sve_contiguous_imm_validates() {
    let instruction = ldnf1w_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1W SVE contiguous immediate should validate");
}

#[test]
fn ldnf1w_sve_contiguous_imm_structure() {
    let instruction = ldnf1w_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, non_fault, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LDNF1W"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*msize, BitWidth::W32);
    assert_eq!(*non_fault, true);
    assert_eq!(*contiguous, true);
}

#[test]
fn ldnf1d_sve_contiguous_imm_validates() {
    let instruction = ldnf1d_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1D SVE contiguous immediate should validate");
}

#[test]
fn ldnf1d_sve_contiguous_imm_structure() {
    let instruction = ldnf1d_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, non_fault, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LDNF1D"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*msize, BitWidth::W64);
    assert_eq!(*non_fault, true);
    assert_eq!(*contiguous, true);
}

#[test]
fn ldnf1sb_sve_contiguous_imm_validates() {
    let instruction = ldnf1sb_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1SB SVE contiguous immediate should validate");
}

#[test]
fn ldnf1sb_sve_contiguous_imm_structure() {
    let instruction = ldnf1sb_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, non_fault, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LDNF1SB"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*data_unsigned, false); // Signed
    assert_eq!(*non_fault, true);
    assert_eq!(*contiguous, true);
}

#[test]
fn ldnf1sh_sve_contiguous_imm_validates() {
    let instruction = ldnf1sh_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1SH SVE contiguous immediate should validate");
}

#[test]
fn ldnf1sh_sve_contiguous_imm_structure() {
    let instruction = ldnf1sh_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, non_fault, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LDNF1SH"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*msize, BitWidth::W16);
    assert_eq!(*data_unsigned, false);
    assert_eq!(*non_fault, true);
    assert_eq!(*contiguous, true);
}

#[test]
fn ldnf1sw_sve_contiguous_imm_validates() {
    let instruction = ldnf1sw_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LDNF1SW SVE contiguous immediate should validate");
}

#[test]
fn ldnf1sw_sve_contiguous_imm_structure() {
    let instruction = ldnf1sw_sve_contiguous_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, non_fault, contiguous, .. } = value else { panic!("expected SVEGatherScatter for LDNF1SW"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad);
    assert_eq!(*msize, BitWidth::W32);
    assert_eq!(*data_unsigned, false);
    assert_eq!(*non_fault, true);
    assert_eq!(*contiguous, true);
}