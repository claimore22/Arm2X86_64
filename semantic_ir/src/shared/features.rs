//! Stanford ARM64 Shared Pseudocode - Feature Detection
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! Stanford functions: IsFeatureImplemented, HaveEL, ELUsingAArch32, etc.

use crate::shared::enums::{ExceptionLevel, SecurityState};

/// ARM architecture feature identifiers
///
/// These correspond to the FEAT_* identifiers used in the Stanford pseudocode
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmFeature {
    // Base architecture
    FEAT_AA64,
    FEAT_AA32,

    // Exception levels
    FEAT_EL2,
    FEAT_EL3,

    // Security
    FEAT_RME,      // Realm Management Extension
    FEAT_SEL2,     // Secure EL2
    FEAT_E2H,      // Enhanced Host (HCR_EL2.E2H)

    // Debug
    FEAT_Debugv8p2,
    FEAT_Debugv8p4,
    FEAT_Debugv8p9,

    // Pointer Authentication
    FEAT_PAuth,
    FEAT_PAuth_LR,
    FEAT_PAuth2,
    FEAT_FPAC,
    FEAT_FPACCOMBINE,

    // Memory Tagging
    FEAT_MTE,
    FEAT_MTE2,
    FEAT_MTE3,
    FEAT_MTE_ASYM,
    FEAT_MTE_PERM,

    // Branch Record Buffer Extension
    FEAT_BRBE,

    // Statistical Profiling
    FEAT_SPE,
    FEAT_SPEv1p1,
    FEAT_SPEv1p2,

    // Scalable Vector Extension
    FEAT_SVE,
    FEAT_SVE2,
    FEAT_SVE_AES,
    FEAT_SVE_BITPERM,
    FEAT_SVE_PMULL128,
    FEAT_SVE_SHA3,
    FEAT_SVE_SM4,
    FEAT_I8MM,
    FEAT_BF16,
    FEAT_FP8,

    // Scalable Matrix Extension
    FEAT_SME,
    FEAT_SME2,
    FEAT_SME_FA64,
    FEAT_SME_F16F16,
    FEAT_SME_I16I16,
    FEAT_SME_B16B16,

    // Memory Model
    FEAT_LOR,
    FEAT_LRCPC,
    FEAT_LRCPC2,
    FEAT_LRCPC3,
    FEAT_LSE,
    FEAT_LSE2,
    FEAT_LSE128,

    // Atomics
    FEAT_ATOMICS,

    // RAS
    FEAT_RAS,
    FEAT_RASv1p1,
    FEAT_RASv2,

    // Counter/Timers
    FEAT_ECV,
    FEAT_ECV_POFF,
    FEAT_CNTSC,

    // FP/SIMD
    FEAT_FP,
    FEAT_SIMD,
    FEAT_FHM,
    FEAT_BF16_FP,
    FEAT_FP16,
    FEAT_FP8_FP,

    // Crypto
    FEAT_AES,
    FEAT_SHA1,
    FEAT_SHA256,
    FEAT_SHA512,
    FEAT_SHA3,
    FEAT_SM3,
    FEAT_SM4,

    // Virtualization
    FEAT_VHE,
    FEAT_NV,
    FEAT_NV2,
    FEAT_HPDS,
    FEAT_HPDS2,
    FEAT_TME,
    FEAT_TME2,

    // Memory Protection
    FEAT_PAN,
    FEAT_PAN2,
    FEAT_PAN3,
    FEAT_APA,
    FEAT_API,
    FEAT_THE,
    FEAT_GCS,
    FEAT_BTI,

    // TLB
    FEAT_TTL,
    FEAT_TLBIOS,
    FEAT_TLBIRANGE,

    // PMU
    FEAT_PMUv3,
    FEAT_PMUv3p1,
    FEAT_PMUv3p4,
    FEAT_PMUv3p5,
    FEAT_PMUv3p7,
    FEAT_PMUv3p8,

    // SVE/SME PMU
    FEAT_SVE_PMU,
    FEAT_SME_PMU,

    // Other
    FEAT_CRC32,
    FEAT_RNG,
    FEAT_RNG_TRAP,
    FEAT_SSBS,
    FEAT_CSV2,
    FEAT_CSV3,
    FEAT_E0PD,
    FEAT_E0PD2,
    FEAT_XS,
    FEAT_HAFDBS,
    FEAT_MOPS,
    FEAT_MOPS_FP16,
    FEAT_MTE_ASYNC,
    FEAT_MTE_SYNC,
    FEAT_DGH,
    FEAT_SB,
    FEAT_LS64,
    FEAT_LS64_V,
    FEAT_LS64_ACCDATA,
    FEAT_FGT,
    FEAT_FGT2,
    FEAT_ETE,
    FEAT_TRBE,
    FEAT_TRF,
    FEAT_ETE2,
    FEAT_ECBHB,
    FEAT_RPRES,
    FEAT_DSLL,
    FEAT_DCT,
    FEAT_AMUv1,
    FEAT_AMUv1p1,
    FEAT_CCIDX,
    FEAT_MPAM,
    FEAT_MPAMv1p1,
    FEAT_HCX,
    FEAT_XNX,
    FEAT_STAGE2_VMSA,
    FEAT_S2FWB,
    FEAT_S2POE,
    FEAT_HAFT,
    FEAT_HPMN0,
    FEAT_CNTPOFF,
    FEAT_CNV,
    FEAT_TIDCP1,
    FEAT_DebugROB,
    FEAT_AIESB,
    FEAT_SVE_EBF16,
    FEAT_SME_BF16,
    FEAT_F32MM,
    FEAT_F64MM,
    FEAT_I8MM_SME,
    FEAT_BF16_SME,
    FEAT_FP8_SME,
}

impl ArmFeature {
    /// Get the feature name as a string
    pub const fn name(self) -> &'static str {
        match self {
            ArmFeature::FEAT_AA64 => "FEAT_AA64",
            ArmFeature::FEAT_AA32 => "FEAT_AA32",
            ArmFeature::FEAT_EL2 => "FEAT_EL2",
            ArmFeature::FEAT_EL3 => "FEAT_EL3",
            ArmFeature::FEAT_RME => "FEAT_RME",
            ArmFeature::FEAT_SEL2 => "FEAT_SEL2",
            ArmFeature::FEAT_E2H => "FEAT_E2H",
            ArmFeature::FEAT_Debugv8p2 => "FEAT_Debugv8p2",
            ArmFeature::FEAT_Debugv8p4 => "FEAT_Debugv8p4",
            ArmFeature::FEAT_Debugv8p9 => "FEAT_Debugv8p9",
            ArmFeature::FEAT_PAuth => "FEAT_PAuth",
            ArmFeature::FEAT_PAuth_LR => "FEAT_PAuth_LR",
            ArmFeature::FEAT_PAuth2 => "FEAT_PAuth2",
            ArmFeature::FEAT_FPAC => "FEAT_FPAC",
            ArmFeature::FEAT_FPACCOMBINE => "FEAT_FPACCOMBINE",
            ArmFeature::FEAT_MTE => "FEAT_MTE",
            ArmFeature::FEAT_MTE2 => "FEAT_MTE2",
            ArmFeature::FEAT_MTE3 => "FEAT_MTE3",
            ArmFeature::FEAT_MTE_ASYM => "FEAT_MTE_ASYM",
            ArmFeature::FEAT_MTE_PERM => "FEAT_MTE_PERM",
            ArmFeature::FEAT_BRBE => "FEAT_BRBE",
            ArmFeature::FEAT_SPE => "FEAT_SPE",
            ArmFeature::FEAT_SPEv1p1 => "FEAT_SPEv1p1",
            ArmFeature::FEAT_SPEv1p2 => "FEAT_SPEv1p2",
            ArmFeature::FEAT_SVE => "FEAT_SVE",
            ArmFeature::FEAT_SVE2 => "FEAT_SVE2",
            ArmFeature::FEAT_SVE_AES => "FEAT_SVE_AES",
            ArmFeature::FEAT_SVE_BITPERM => "FEAT_SVE_BITPERM",
            ArmFeature::FEAT_SVE_PMULL128 => "FEAT_SVE_PMULL128",
            ArmFeature::FEAT_SVE_SHA3 => "FEAT_SVE_SHA3",
            ArmFeature::FEAT_SVE_SM4 => "FEAT_SVE_SM4",
            ArmFeature::FEAT_I8MM => "FEAT_I8MM",
            ArmFeature::FEAT_BF16 => "FEAT_BF16",
            ArmFeature::FEAT_FP8 => "FEAT_FP8",
            ArmFeature::FEAT_SME => "FEAT_SME",
            ArmFeature::FEAT_SME2 => "FEAT_SME2",
            ArmFeature::FEAT_SME_FA64 => "FEAT_SME_FA64",
            ArmFeature::FEAT_SME_F16F16 => "FEAT_SME_F16F16",
            ArmFeature::FEAT_SME_I16I16 => "FEAT_SME_I16I16",
            ArmFeature::FEAT_SME_B16B16 => "FEAT_SME_B16B16",
            ArmFeature::FEAT_LOR => "FEAT_LOR",
            ArmFeature::FEAT_LRCPC => "FEAT_LRCPC",
            ArmFeature::FEAT_LRCPC2 => "FEAT_LRCPC2",
            ArmFeature::FEAT_LRCPC3 => "FEAT_LRCPC3",
            ArmFeature::FEAT_LSE => "FEAT_LSE",
            ArmFeature::FEAT_LSE2 => "FEAT_LSE2",
            ArmFeature::FEAT_LSE128 => "FEAT_LSE128",
            ArmFeature::FEAT_ATOMICS => "FEAT_ATOMICS",
            ArmFeature::FEAT_RAS => "FEAT_RAS",
            ArmFeature::FEAT_RASv1p1 => "FEAT_RASv1p1",
            ArmFeature::FEAT_RASv2 => "FEAT_RASv2",
            ArmFeature::FEAT_ECV => "FEAT_ECV",
            ArmFeature::FEAT_ECV_POFF => "FEAT_ECV_POFF",
            ArmFeature::FEAT_CNTSC => "FEAT_CNTSC",
            ArmFeature::FEAT_FP => "FEAT_FP",
            ArmFeature::FEAT_SIMD => "FEAT_SIMD",
            ArmFeature::FEAT_FHM => "FEAT_FHM",
            ArmFeature::FEAT_BF16_FP => "FEAT_BF16_FP",
            ArmFeature::FEAT_FP16 => "FEAT_FP16",
            ArmFeature::FEAT_FP8_FP => "FEAT_FP8_FP",
            ArmFeature::FEAT_AES => "FEAT_AES",
            ArmFeature::FEAT_SHA1 => "FEAT_SHA1",
            ArmFeature::FEAT_SHA256 => "FEAT_SHA256",
            ArmFeature::FEAT_SHA512 => "FEAT_SHA512",
            ArmFeature::FEAT_SHA3 => "FEAT_SHA3",
            ArmFeature::FEAT_SM3 => "FEAT_SM3",
            ArmFeature::FEAT_SM4 => "FEAT_SM4",
            ArmFeature::FEAT_VHE => "FEAT_VHE",
            ArmFeature::FEAT_NV => "FEAT_NV",
            ArmFeature::FEAT_NV2 => "FEAT_NV2",
            ArmFeature::FEAT_HPDS => "FEAT_HPDS",
            ArmFeature::FEAT_HPDS2 => "FEAT_HPDS2",
            ArmFeature::FEAT_TME => "FEAT_TME",
            ArmFeature::FEAT_TME2 => "FEAT_TME2",
            ArmFeature::FEAT_PAN => "FEAT_PAN",
            ArmFeature::FEAT_PAN2 => "FEAT_PAN2",
            ArmFeature::FEAT_PAN3 => "FEAT_PAN3",
            ArmFeature::FEAT_APA => "FEAT_APA",
            ArmFeature::FEAT_API => "FEAT_API",
            ArmFeature::FEAT_THE => "FEAT_THE",
            ArmFeature::FEAT_GCS => "FEAT_GCS",
            ArmFeature::FEAT_BTI => "FEAT_BTI",
            ArmFeature::FEAT_TTL => "FEAT_TTL",
            ArmFeature::FEAT_TLBIOS => "FEAT_TLBIOS",
            ArmFeature::FEAT_TLBIRANGE => "FEAT_TLBIRANGE",
            ArmFeature::FEAT_PMUv3 => "FEAT_PMUv3",
            ArmFeature::FEAT_PMUv3p1 => "FEAT_PMUv3p1",
            ArmFeature::FEAT_PMUv3p4 => "FEAT_PMUv3p4",
            ArmFeature::FEAT_PMUv3p5 => "FEAT_PMUv3p5",
            ArmFeature::FEAT_PMUv3p7 => "FEAT_PMUv3p7",
            ArmFeature::FEAT_PMUv3p8 => "FEAT_PMUv3p8",
            ArmFeature::FEAT_SVE_PMU => "FEAT_SVE_PMU",
            ArmFeature::FEAT_SME_PMU => "FEAT_SME_PMU",
            ArmFeature::FEAT_CRC32 => "FEAT_CRC32",
            ArmFeature::FEAT_RNG => "FEAT_RNG",
            ArmFeature::FEAT_RNG_TRAP => "FEAT_RNG_TRAP",
            ArmFeature::FEAT_SSBS => "FEAT_SSBS",
            ArmFeature::FEAT_CSV2 => "FEAT_CSV2",
            ArmFeature::FEAT_CSV3 => "FEAT_CSV3",
            ArmFeature::FEAT_E0PD => "FEAT_E0PD",
            ArmFeature::FEAT_E0PD2 => "FEAT_E0PD2",
            ArmFeature::FEAT_XS => "FEAT_XS",
            ArmFeature::FEAT_HAFDBS => "FEAT_HAFDBS",
            ArmFeature::FEAT_MOPS => "FEAT_MOPS",
            ArmFeature::FEAT_MOPS_FP16 => "FEAT_MOPS_FP16",
            ArmFeature::FEAT_MTE_ASYNC => "FEAT_MTE_ASYNC",
            ArmFeature::FEAT_MTE_SYNC => "FEAT_MTE_SYNC",
            ArmFeature::FEAT_DGH => "FEAT_DGH",
            ArmFeature::FEAT_SB => "FEAT_SB",
            ArmFeature::FEAT_LS64 => "FEAT_LS64",
            ArmFeature::FEAT_LS64_V => "FEAT_LS64_V",
            ArmFeature::FEAT_LS64_ACCDATA => "FEAT_LS64_ACCDATA",
            ArmFeature::FEAT_FGT => "FEAT_FGT",
            ArmFeature::FEAT_FGT2 => "FEAT_FGT2",
            ArmFeature::FEAT_ETE => "FEAT_ETE",
            ArmFeature::FEAT_TRBE => "FEAT_TRBE",
            ArmFeature::FEAT_TRF => "FEAT_TRF",
            ArmFeature::FEAT_ETE2 => "FEAT_ETE2",
            ArmFeature::FEAT_ECBHB => "FEAT_ECBHB",
            ArmFeature::FEAT_RPRES => "FEAT_RPRES",
            ArmFeature::FEAT_DSLL => "FEAT_DSLL",
            ArmFeature::FEAT_DCT => "FEAT_DCT",
            ArmFeature::FEAT_AMUv1 => "FEAT_AMUv1",
            ArmFeature::FEAT_AMUv1p1 => "FEAT_AMUv1p1",
            ArmFeature::FEAT_CCIDX => "FEAT_CCIDX",
            ArmFeature::FEAT_MPAM => "FEAT_MPAM",
            ArmFeature::FEAT_MPAMv1p1 => "FEAT_MPAMv1p1",
            ArmFeature::FEAT_HCX => "FEAT_HCX",
            ArmFeature::FEAT_XNX => "FEAT_XNX",
            ArmFeature::FEAT_STAGE2_VMSA => "FEAT_STAGE2_VMSA",
            ArmFeature::FEAT_S2FWB => "FEAT_S2FWB",
            ArmFeature::FEAT_S2POE => "FEAT_S2POE",
            ArmFeature::FEAT_HAFT => "FEAT_HAFT",
            ArmFeature::FEAT_HPMN0 => "FEAT_HPMN0",
            ArmFeature::FEAT_CNTPOFF => "FEAT_CNTPOFF",
            ArmFeature::FEAT_CNV => "FEAT_CNV",
            ArmFeature::FEAT_TIDCP1 => "FEAT_TIDCP1",
            ArmFeature::FEAT_DebugROB => "FEAT_DebugROB",
            ArmFeature::FEAT_AIESB => "FEAT_AIESB",
            ArmFeature::FEAT_SVE_EBF16 => "FEAT_SVE_EBF16",
            ArmFeature::FEAT_SME_BF16 => "FEAT_SME_BF16",
            ArmFeature::FEAT_F32MM => "FEAT_F32MM",
            ArmFeature::FEAT_F64MM => "FEAT_F64MM",
            ArmFeature::FEAT_I8MM_SME => "FEAT_I8MM_SME",
            ArmFeature::FEAT_BF16_SME => "FEAT_BF16_SME",
            ArmFeature::FEAT_FP8_SME => "FEAT_FP8_SME",
        }
    }
}

/// CPU feature register state
///
/// This represents the ID_AA64PFR0_EL1, ID_AA64PFR1_EL1, etc. registers
/// that determine which features are implemented.
#[derive(Debug, Clone, Default)]
pub struct FeatureRegisters {
    /// ID_AA64PFR0_EL1
    pub id_aa64pfr0_el1: u64,
    /// ID_AA64PFR1_EL1
    pub id_aa64pfr1_el1: u64,
    /// ID_AA64ISAR0_EL1
    pub id_aa64isar0_el1: u64,
    /// ID_AA64ISAR1_EL1
    pub id_aa64isar1_el1: u64,
    /// ID_AA64ISAR2_EL1
    pub id_aa64isar2_el1: u64,
    /// ID_AA64MMFR0_EL1
    pub id_aa64mmfr0_el1: u64,
    /// ID_AA64MMFR1_EL1
    pub id_aa64mmfr1_el1: u64,
    /// ID_AA64MMFR2_EL1
    pub id_aa64mmfr2_el1: u64,
    /// ID_AA64MMFR3_EL1
    pub id_aa64mmfr3_el1: u64,
    /// ID_AA64SMFR0_EL1
    pub id_aa64smfr0_el1: u64,
    /// ID_AA64ZFR0_EL1
    pub id_aa64zfr0_el1: u64,
    /// ID_AA64DFR0_EL1
    pub id_aa64dfr0_el1: u64,
    /// ID_AA64DFR1_EL1
    pub id_aa64dfr1_el1: u64,
    /// MVFR0_EL1
    pub mvfr0_el1: u64,
    /// MVFR1_EL1
    pub mvfr1_el1: u64,
    /// MVFR2_EL1
    pub mvfr2_el1: u64,
}

impl FeatureRegisters {
    /// Create a new FeatureRegisters with all features enabled (for testing)
    pub fn all_features() -> Self {
        Self {
            id_aa64pfr0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64pfr1_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64isar0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64isar1_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64isar2_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64mmfr0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64mmfr1_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64mmfr2_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64mmfr3_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64smfr0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64zfr0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64dfr0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            id_aa64dfr1_el1: 0xFFFF_FFFF_FFFF_FFFF,
            mvfr0_el1: 0xFFFF_FFFF_FFFF_FFFF,
            mvfr1_el1: 0xFFFF_FFFF_FFFF_FFFF,
            mvfr2_el1: 0xFFFF_FFFF_FFFF_FFFF,
        }
    }

    /// Create a minimal feature set (base AArch64 only)
    pub fn base_aarch64() -> Self {
        Self {
            id_aa64pfr0_el1: 0x0000_1131, // EL0, EL1, EL2, EL3, FP, AdvSIMD
            id_aa64pfr1_el1: 0x0000_0011, // MTE, SVE, SME
            id_aa64isar0_el1: 0x0111_1111, // Basic instructions
            id_aa64mmfr0_el1: 0x1111_1111, // Basic MMU
            ..Default::default()
        }
    }
}

use std::sync::OnceLock;

/// Global feature state (in a real implementation, this would be per-CPU)
static FEATURE_REGISTERS: OnceLock<FeatureRegisters> = OnceLock::new();

/// Set the global feature registers (for testing/initialization)
pub fn set_feature_registers(regs: FeatureRegisters) {
    let _ = FEATURE_REGISTERS.set(regs);
}

/// Get the global feature registers
fn get_feature_registers() -> &'static FeatureRegisters {
    FEATURE_REGISTERS.get_or_init(FeatureRegisters::base_aarch64)
}

/// Stanford pseudocode: IsFeatureImplemented
///
/// ```text
/// func IsFeatureImplemented(feature : Feature) => boolean
/// begin
///     return feature is implemented in this implementation;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsFeatureImplemented
pub fn is_feature_implemented(feature: ArmFeature) -> bool {
    // In a real implementation, this would check the appropriate ID register fields
    // For now, we use a simplified mapping
    match feature {
        // Base architecture
        ArmFeature::FEAT_AA64 => true,
        ArmFeature::FEAT_AA32 => false, // This project is AArch64-only

        // Exception levels
        ArmFeature::FEAT_EL2 => have_el(ExceptionLevel::EL2),
        ArmFeature::FEAT_EL3 => have_el(ExceptionLevel::EL3),

        // Security
        ArmFeature::FEAT_RME => false, // Not implemented by default
        ArmFeature::FEAT_SEL2 => false,
        ArmFeature::FEAT_E2H => false,

        // Debug
        ArmFeature::FEAT_Debugv8p2 => false,
        ArmFeature::FEAT_Debugv8p4 => false,
        ArmFeature::FEAT_Debugv8p9 => false,

        // Pointer Authentication
        ArmFeature::FEAT_PAuth => false,
        ArmFeature::FEAT_PAuth_LR => false,
        ArmFeature::FEAT_PAuth2 => false,
        ArmFeature::FEAT_FPAC => false,
        ArmFeature::FEAT_FPACCOMBINE => false,

        // Memory Tagging
        ArmFeature::FEAT_MTE => false,
        ArmFeature::FEAT_MTE2 => false,
        ArmFeature::FEAT_MTE3 => false,
        ArmFeature::FEAT_MTE_ASYM => false,
        ArmFeature::FEAT_MTE_PERM => false,

        // BRBE
        ArmFeature::FEAT_BRBE => false,

        // SPE
        ArmFeature::FEAT_SPE => false,
        ArmFeature::FEAT_SPEv1p1 => false,
        ArmFeature::FEAT_SPEv1p2 => false,

        // SVE
        ArmFeature::FEAT_SVE => false,
        ArmFeature::FEAT_SVE2 => false,
        ArmFeature::FEAT_SVE_AES => false,
        ArmFeature::FEAT_SVE_BITPERM => false,
        ArmFeature::FEAT_SVE_PMULL128 => false,
        ArmFeature::FEAT_SVE_SHA3 => false,
        ArmFeature::FEAT_SVE_SM4 => false,
        ArmFeature::FEAT_I8MM => false,
        ArmFeature::FEAT_BF16 => false,
        ArmFeature::FEAT_FP8 => false,

        // SME
        ArmFeature::FEAT_SME => false,
        ArmFeature::FEAT_SME2 => false,
        ArmFeature::FEAT_SME_FA64 => false,
        ArmFeature::FEAT_SME_F16F16 => false,
        ArmFeature::FEAT_SME_I16I16 => false,
        ArmFeature::FEAT_SME_B16B16 => false,

        // Memory Model
        ArmFeature::FEAT_LOR => false,
        ArmFeature::FEAT_LRCPC => false,
        ArmFeature::FEAT_LRCPC2 => false,
        ArmFeature::FEAT_LRCPC3 => false,
        ArmFeature::FEAT_LSE => false,
        ArmFeature::FEAT_LSE2 => false,
        ArmFeature::FEAT_LSE128 => false,

        // Atomics
        ArmFeature::FEAT_ATOMICS => false,

        // RAS
        ArmFeature::FEAT_RAS => false,
        ArmFeature::FEAT_RASv1p1 => false,
        ArmFeature::FEAT_RASv2 => false,

        // Counter/Timers
        ArmFeature::FEAT_ECV => false,
        ArmFeature::FEAT_ECV_POFF => false,
        ArmFeature::FEAT_CNTSC => false,

        // FP/SIMD
        ArmFeature::FEAT_FP => true,   // Base AArch64 includes FP
        ArmFeature::FEAT_SIMD => true, // Base AArch64 includes AdvSIMD
        ArmFeature::FEAT_FHM => false,
        ArmFeature::FEAT_BF16_FP => false,
        ArmFeature::FEAT_FP16 => true,
        ArmFeature::FEAT_FP8_FP => false,

        // Crypto
        ArmFeature::FEAT_AES => false,
        ArmFeature::FEAT_SHA1 => false,
        ArmFeature::FEAT_SHA256 => false,
        ArmFeature::FEAT_SHA512 => false,
        ArmFeature::FEAT_SHA3 => false,
        ArmFeature::FEAT_SM3 => false,
        ArmFeature::FEAT_SM4 => false,

        // Virtualization
        ArmFeature::FEAT_VHE => false,
        ArmFeature::FEAT_NV => false,
        ArmFeature::FEAT_NV2 => false,
        ArmFeature::FEAT_HPDS => false,
        ArmFeature::FEAT_HPDS2 => false,
        ArmFeature::FEAT_TME => false,
        ArmFeature::FEAT_TME2 => false,

        // Memory Protection
        ArmFeature::FEAT_PAN => false,
        ArmFeature::FEAT_PAN2 => false,
        ArmFeature::FEAT_PAN3 => false,
        ArmFeature::FEAT_APA => false,
        ArmFeature::FEAT_API => false,
        ArmFeature::FEAT_THE => false,
        ArmFeature::FEAT_GCS => false,
        ArmFeature::FEAT_BTI => false,

        // TLB
        ArmFeature::FEAT_TTL => false,
        ArmFeature::FEAT_TLBIOS => false,
        ArmFeature::FEAT_TLBIRANGE => false,

        // PMU
        ArmFeature::FEAT_PMUv3 => false,
        ArmFeature::FEAT_PMUv3p1 => false,
        ArmFeature::FEAT_PMUv3p4 => false,
        ArmFeature::FEAT_PMUv3p5 => false,
        ArmFeature::FEAT_PMUv3p7 => false,
        ArmFeature::FEAT_PMUv3p8 => false,

        // SVE/SME PMU
        ArmFeature::FEAT_SVE_PMU => false,
        ArmFeature::FEAT_SME_PMU => false,

        // Other
        ArmFeature::FEAT_CRC32 => false,
        ArmFeature::FEAT_RNG => false,
        ArmFeature::FEAT_RNG_TRAP => false,
        ArmFeature::FEAT_SSBS => false,
        ArmFeature::FEAT_CSV2 => false,
        ArmFeature::FEAT_CSV3 => false,
        ArmFeature::FEAT_E0PD => false,
        ArmFeature::FEAT_E0PD2 => false,
        ArmFeature::FEAT_XS => false,
        ArmFeature::FEAT_HAFDBS => false,
        ArmFeature::FEAT_MOPS => false,
        ArmFeature::FEAT_MOPS_FP16 => false,
        ArmFeature::FEAT_MTE_ASYNC => false,
        ArmFeature::FEAT_MTE_SYNC => false,
        ArmFeature::FEAT_DGH => false,
        ArmFeature::FEAT_SB => false,
        ArmFeature::FEAT_LS64 => false,
        ArmFeature::FEAT_LS64_V => false,
        ArmFeature::FEAT_LS64_ACCDATA => false,
        ArmFeature::FEAT_FGT => false,
        ArmFeature::FEAT_FGT2 => false,
        ArmFeature::FEAT_ETE => false,
        ArmFeature::FEAT_TRBE => false,
        ArmFeature::FEAT_TRF => false,
        ArmFeature::FEAT_ETE2 => false,
        ArmFeature::FEAT_ECBHB => false,
        ArmFeature::FEAT_RPRES => false,
        ArmFeature::FEAT_DSLL => false,
        ArmFeature::FEAT_DCT => false,
        ArmFeature::FEAT_AMUv1 => false,
        ArmFeature::FEAT_AMUv1p1 => false,
        ArmFeature::FEAT_CCIDX => false,
        ArmFeature::FEAT_MPAM => false,
        ArmFeature::FEAT_MPAMv1p1 => false,
        ArmFeature::FEAT_HCX => false,
        ArmFeature::FEAT_XNX => false,
        ArmFeature::FEAT_STAGE2_VMSA => false,
        ArmFeature::FEAT_S2FWB => false,
        ArmFeature::FEAT_S2POE => false,
        ArmFeature::FEAT_HAFT => false,
        ArmFeature::FEAT_HPMN0 => false,
        ArmFeature::FEAT_CNTPOFF => false,
        ArmFeature::FEAT_CNV => false,
        ArmFeature::FEAT_TIDCP1 => false,
        ArmFeature::FEAT_DebugROB => false,
        ArmFeature::FEAT_AIESB => false,
        ArmFeature::FEAT_SVE_EBF16 => false,
        ArmFeature::FEAT_SME_BF16 => false,
        ArmFeature::FEAT_F32MM => false,
        ArmFeature::FEAT_F64MM => false,
        ArmFeature::FEAT_I8MM_SME => false,
        ArmFeature::FEAT_BF16_SME => false,
        ArmFeature::FEAT_FP8_SME => false,
    }
}

/// Stanford pseudocode: HaveEL
///
/// ```text
/// func HaveEL(el : bits(2)) => boolean
/// begin
///     return EL is implemented;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// HaveEL
pub fn have_el(el: ExceptionLevel) -> bool {
    match el {
        ExceptionLevel::EL0 => true,
        ExceptionLevel::EL1 => true,
        ExceptionLevel::EL2 => is_feature_implemented(ArmFeature::FEAT_EL2),
        ExceptionLevel::EL3 => is_feature_implemented(ArmFeature::FEAT_EL3),
    }
}

/// Stanford pseudocode: ELUsingAArch32
///
/// ```text
/// func ELUsingAArch32(el : bits(2)) => boolean
/// begin
///     return EL is using AArch32;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ELUsingAArch32
pub fn el_using_aarch32(el: ExceptionLevel) -> bool {
    // This project is AArch64-only, so no EL uses AArch32
    false
}

/// Stanford pseudocode: UsingAArch32
///
/// ```text
/// func UsingAArch32() => boolean
/// begin
///     return CurrentEL is using AArch32;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// UsingAArch32
pub fn using_aarch32() -> bool {
    false
}

/// Stanford pseudocode: HaveAArch64
///
/// ```text
/// func HaveAArch64() => boolean
/// begin
///     return AArch64 is implemented;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// HaveAArch64
pub fn have_aarch64() -> bool {
    true
}

/// Stanford pseudocode: HaveAArch32
///
/// ```text
/// func HaveAArch32() => boolean
/// begin
///     return AArch32 is implemented;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// HaveAArch32
pub fn have_aarch32() -> bool {
    false
}

/// Stanford pseudocode: IsSecureEL2Enabled
///
/// ```text
/// func IsSecureEL2Enabled() => boolean
/// begin
///     return HaveEL(EL3) && SCR_EL3.EEL2 == '1';
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsSecureEL2Enabled
pub fn is_secure_el2_enabled() -> bool {
    have_el(ExceptionLevel::EL3) && is_feature_implemented(ArmFeature::FEAT_SEL2)
}

/// Stanford pseudocode: IsInHost
///
/// ```text
/// func IsInHost() => boolean
/// begin
///     return PSTATE.EL == EL2 && HCR_EL2.E2H == '1';
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsInHost
pub fn is_in_host(current_el: ExceptionLevel, hcr_el2_e2h: bool) -> bool {
    current_el == ExceptionLevel::EL2 && hcr_el2_e2h
}

/// Stanford pseudocode: ELIsInHost
///
/// ```text
/// func ELIsInHost(el : bits(2)) => boolean
/// begin
///     return el == EL2 && HCR_EL2.E2H == '1';
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ELIsInHost
pub fn el_is_in_host(el: ExceptionLevel, hcr_el2_e2h: bool) -> bool {
    el == ExceptionLevel::EL2 && hcr_el2_e2h
}

/// Stanford pseudocode: CurrentSecurityState
///
/// ```text
/// func CurrentSecurityState() => SecurityState
/// begin
///     // Returns the current security state based on EL and SCR_EL3
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// CurrentSecurityState
pub fn current_security_state(
    current_el: ExceptionLevel,
    scr_el3_ns: bool,
    is_secure_el2: bool,
) -> SecurityState {
    if is_feature_implemented(ArmFeature::FEAT_RME) {
        // FEAT_RME: 4 security states
        // This is a simplified implementation
        SecurityState::Secure
    } else if have_el(ExceptionLevel::EL3) {
        if current_el == ExceptionLevel::EL3 {
            SecurityState::Secure
        } else if scr_el3_ns {
            SecurityState::NonSecure
        } else {
            SecurityState::Secure
        }
    } else {
        SecurityState::NonSecure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_have_el() {
        assert!(have_el(ExceptionLevel::EL0));
        assert!(have_el(ExceptionLevel::EL1));
        // EL2 and EL3 depend on features
    }

    #[test]
    fn test_el_using_aarch32() {
        assert!(!el_using_aarch32(ExceptionLevel::EL0));
        assert!(!el_using_aarch32(ExceptionLevel::EL1));
        assert!(!el_using_aarch32(ExceptionLevel::EL2));
        assert!(!el_using_aarch32(ExceptionLevel::EL3));
    }

    #[test]
    fn test_using_aarch32() {
        assert!(!using_aarch32());
    }

    #[test]
    fn test_have_aarch64() {
        assert!(have_aarch64());
    }

    #[test]
    fn test_have_aarch32() {
        assert!(!have_aarch32());
    }

    #[test]
    fn test_is_feature_implemented_base() {
        assert!(is_feature_implemented(ArmFeature::FEAT_AA64));
        assert!(is_feature_implemented(ArmFeature::FEAT_FP));
        assert!(is_feature_implemented(ArmFeature::FEAT_SIMD));
        assert!(is_feature_implemented(ArmFeature::FEAT_FP16));
    }

    #[test]
    fn test_is_feature_implemented_optional() {
        // These are not implemented by default
        assert!(!is_feature_implemented(ArmFeature::FEAT_RME));
        assert!(!is_feature_implemented(ArmFeature::FEAT_SVE));
        assert!(!is_feature_implemented(ArmFeature::FEAT_SME));
        assert!(!is_feature_implemented(ArmFeature::FEAT_MTE));
        assert!(!is_feature_implemented(ArmFeature::FEAT_PAuth));
    }
}