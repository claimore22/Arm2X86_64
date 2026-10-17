/// FRINT<r> family (SVE predicated) - Floating-point round to integral value
/// Stanford source: arm64_md_clean/frinta_z_p_z.md (covers all FRINT<r> variants)
///
/// FRINT<r> variants with different rounding modes:
/// - FRINTN: to nearest, ties to even (FPRounding_TIEEVEN)
/// - FRINTA: to nearest, ties away from zero (FPRounding_POSINF)
/// - FRINTM: toward minus infinity (FPRounding_NEGINF)
/// - FRINTP: toward plus infinity (FPRounding_POSINF)
/// - FRINTZ: toward zero (FPRounding_ZERO)
/// - FRINTI: current FPCR rounding mode (FPRoundingMode(FPCR))
/// - FRINTX: current FPCR rounding mode, signalling inexact
/// - FRINT32X: round to 32-bit integer, current FPCR mode
/// - FRINT32Z: round to 32-bit integer, toward zero
/// - FRINT64X: round to 64-bit integer, current FPCR mode
/// - FRINT64Z: round to 64-bit integer, toward zero
///
/// All have merging and zeroing predication variants.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn create_frint_instruction(mnemonic: &str, op: semantic_ir::primitives::SVEVectorUnaryOp, esize: BitWidth, feature: &str, predication: semantic_ir::primitives::PredicationMode) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Floating-point round to integral value ({})", mnemonic)),
        },
        operands: Vec::new(),
        required_features: vec![feature.to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frinta_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn frintn_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTN", semantic_ir::primitives::SVEVectorUnaryOp::Frintn, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frintn_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTN", semantic_ir::primitives::SVEVectorUnaryOp::Frintn, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frinta_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTA", semantic_ir::primitives::SVEVectorUnaryOp::Frinta, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frinta_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTA", semantic_ir::primitives::SVEVectorUnaryOp::Frinta, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frintm_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTM", semantic_ir::primitives::SVEVectorUnaryOp::Frintm, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frintm_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTM", semantic_ir::primitives::SVEVectorUnaryOp::Frintm, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frintp_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTP", semantic_ir::primitives::SVEVectorUnaryOp::Frintp, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frintp_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTP", semantic_ir::primitives::SVEVectorUnaryOp::Frintp, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frintz_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTZ", semantic_ir::primitives::SVEVectorUnaryOp::Frintz, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frintz_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTZ", semantic_ir::primitives::SVEVectorUnaryOp::Frintz, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frinti_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTI", semantic_ir::primitives::SVEVectorUnaryOp::Frinti, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frinti_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTI", semantic_ir::primitives::SVEVectorUnaryOp::Frinti, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frintx_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINTX", semantic_ir::primitives::SVEVectorUnaryOp::Frintx, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frintx_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINTX", semantic_ir::primitives::SVEVectorUnaryOp::Frintx, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frint32x_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINT32X", semantic_ir::primitives::SVEVectorUnaryOp::FRint32X, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frint32x_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINT32X", semantic_ir::primitives::SVEVectorUnaryOp::FRint32X, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frint32z_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINT32Z", semantic_ir::primitives::SVEVectorUnaryOp::FRint32Z, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frint32z_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINT32Z", semantic_ir::primitives::SVEVectorUnaryOp::FRint32Z, BitWidth::W32, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frint64x_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINT64X", semantic_ir::primitives::SVEVectorUnaryOp::FRint64X, BitWidth::W64, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frint64x_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINT64X", semantic_ir::primitives::SVEVectorUnaryOp::FRint64X, BitWidth::W64, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

fn frint64z_sve_merging_instruction() -> Instruction {
    create_frint_instruction("FRINT64Z", semantic_ir::primitives::SVEVectorUnaryOp::FRint64Z, BitWidth::W64, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Merging)
}

fn frint64z_sve_zeroing_instruction() -> Instruction {
    create_frint_instruction("FRINT64Z", semantic_ir::primitives::SVEVectorUnaryOp::FRint64Z, BitWidth::W64, "FEAT_SVE2p2", semantic_ir::primitives::PredicationMode::Zeroing)
}

#[test]
fn frintn_sve_merging_validates() {
    let instruction = frintn_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTN SVE merging should validate");
}

#[test]
fn frintn_sve_zeroing_validates() {
    let instruction = frintn_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTN SVE zeroing should validate");
}

#[test]
fn frinta_sve_merging_validates() {
    let instruction = frinta_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTA SVE merging should validate");
}

#[test]
fn frinta_sve_zeroing_validates() {
    let instruction = frinta_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTA SVE zeroing should validate");
}

#[test]
fn frintm_sve_merging_validates() {
    let instruction = frintm_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTM SVE merging should validate");
}

#[test]
fn frintm_sve_zeroing_validates() {
    let instruction = frintm_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTM SVE zeroing should validate");
}

#[test]
fn frintp_sve_merging_validates() {
    let instruction = frintp_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTP SVE merging should validate");
}

#[test]
fn frintp_sve_zeroing_validates() {
    let instruction = frintp_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTP SVE zeroing should validate");
}

#[test]
fn frintz_sve_merging_validates() {
    let instruction = frintz_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTZ SVE merging should validate");
}

#[test]
fn frintz_sve_zeroing_validates() {
    let instruction = frintz_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTZ SVE zeroing should validate");
}

#[test]
fn frinti_sve_merging_validates() {
    let instruction = frinti_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTI SVE merging should validate");
}

#[test]
fn frinti_sve_zeroing_validates() {
    let instruction = frinti_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTI SVE zeroing should validate");
}

#[test]
fn frintx_sve_merging_validates() {
    let instruction = frintx_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINTX SVE merging should validate");
}

#[test]
fn frintx_sve_zeroing_validates() {
    let instruction = frintx_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINTX SVE zeroing should validate");
}

#[test]
fn frint32x_sve_merging_validates() {
    let instruction = frint32x_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINT32X SVE merging should validate");
}

#[test]
fn frint32x_sve_zeroing_validates() {
    let instruction = frint32x_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINT32X SVE zeroing should validate");
}

#[test]
fn frint32z_sve_merging_validates() {
    let instruction = frint32z_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINT32Z SVE merging should validate");
}

#[test]
fn frint32z_sve_zeroing_validates() {
    let instruction = frint32z_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINT32Z SVE zeroing should validate");
}

#[test]
fn frint64x_sve_merging_validates() {
    let instruction = frint64x_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINT64X SVE merging should validate");
}

#[test]
fn frint64x_sve_zeroing_validates() {
    let instruction = frint64x_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINT64X SVE zeroing should validate");
}

#[test]
fn frint64z_sve_merging_validates() {
    let instruction = frint64z_sve_merging_instruction();
    validate_instruction(&instruction).expect("FRINT64Z SVE merging should validate");
}

#[test]
fn frint64z_sve_zeroing_validates() {
    let instruction = frint64z_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FRINT64Z SVE zeroing should validate");
}