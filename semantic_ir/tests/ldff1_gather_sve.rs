/// LDFF1B, LDFF1H, LDFF1W, LDFF1D, LDFF1SB, LDFF1SH, LDFF1SW (SVE) - First-fault gather loads
/// Stanford sources: arm64_md_clean/ldff1*_z_p_*.md
///
/// These instructions perform gather loads with first-faulting behavior:
/// - Only the first active element that causes a fault will actually fault
/// - Subsequent elements after a fault are set to zero (or kept as original)
/// - Used for safe speculative vector loads
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn create_ldff1_gather_instruction(
    mnemonic: &str,
    msize: BitWidth,
    data_unsigned: bool,
) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Vector index elements are 64-bit

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        base: Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 }),
        index: Box::new(Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        vl: BitWidth::W256,
        esize: BitWidth::W64,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned,
        first_fault: true,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl: BitWidth::W256 }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Gather load first-fault {} to vector (vector index)",
                match mnemonic {
                    "LDFF1B" => "unsigned bytes",
                    "LDFF1H" => "unsigned halfwords",
                    "LDFF1W" => "unsigned words",
                    "LDFF1D" => "doublewords",
                    "LDFF1SB" => "signed bytes",
                    "LDFF1SH" => "signed halfwords",
                    "LDFF1SW" => "signed words",
                    _ => "unknown",
                })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl: BitWidth::W256 }],
        provenance: Provenance {
            source_path: format!("ldff1{}_z_p_bz.md", &mnemonic[5..]),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ldff1b_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1B", BitWidth::W8, true)
}

fn ldff1h_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1H", BitWidth::W16, true)
}

fn ldff1w_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1W", BitWidth::W32, true)
}

fn ldff1d_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1D", BitWidth::W64, true)
}

fn ldff1sb_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1SB", BitWidth::W8, false)
}

fn ldff1sh_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1SH", BitWidth::W16, false)
}

fn ldff1sw_sve_gather_instruction() -> Instruction {
    create_ldff1_gather_instruction("LDFF1SW", BitWidth::W32, false)
}

#[test]
fn ldff1b_sve_gather_validates() {
    let instruction = ldff1b_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1B SVE first-fault gather should validate");
}

#[test]
fn ldff1b_sve_gather_structure() {
    let instruction = ldff1b_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1B"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*first_fault, true);
    let Expr::ReadPredReg { .. } = predicate.as_ref() else { panic!("expected ReadPredReg for predicate"); };
    matches!(base.as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 });
    matches!(index.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn ldff1h_sve_gather_validates() {
    let instruction = ldff1h_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1H SVE first-fault gather should validate");
}

#[test]
fn ldff1h_sve_gather_structure() {
    let instruction = ldff1h_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1H"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*msize, BitWidth::W16);
    assert_eq!(*first_fault, true);
}

#[test]
fn ldff1w_sve_gather_validates() {
    let instruction = ldff1w_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1W SVE first-fault gather should validate");
}

#[test]
fn ldff1w_sve_gather_structure() {
    let instruction = ldff1w_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1W"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*msize, BitWidth::W32);
    assert_eq!(*first_fault, true);
}

#[test]
fn ldff1d_sve_gather_validates() {
    let instruction = ldff1d_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1D SVE first-fault gather should validate");
}

#[test]
fn ldff1d_sve_gather_structure() {
    let instruction = ldff1d_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1D"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*msize, BitWidth::W64);
    assert_eq!(*first_fault, true);
}

#[test]
fn ldff1sb_sve_gather_validates() {
    let instruction = ldff1sb_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1SB SVE first-fault gather should validate");
}

#[test]
fn ldff1sb_sve_gather_structure() {
    let instruction = ldff1sb_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1SB"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*data_unsigned, false); // Signed
    assert_eq!(*first_fault, true);
}

#[test]
fn ldff1sh_sve_gather_validates() {
    let instruction = ldff1sh_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1SH SVE first-fault gather should validate");
}

#[test]
fn ldff1sh_sve_gather_structure() {
    let instruction = ldff1sh_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1SH"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*msize, BitWidth::W16);
    assert_eq!(*data_unsigned, false);
    assert_eq!(*first_fault, true);
}

#[test]
fn ldff1sw_sve_gather_validates() {
    let instruction = ldff1sw_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1SW SVE first-fault gather should validate");
}

#[test]
fn ldff1sw_sve_gather_structure() {
    let instruction = ldff1sw_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, msize, data_unsigned, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1SW"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*msize, BitWidth::W32);
    assert_eq!(*data_unsigned, false);
    assert_eq!(*first_fault, true);
}