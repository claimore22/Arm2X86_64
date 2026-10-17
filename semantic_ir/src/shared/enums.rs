//! Stanford ARM64 Shared Pseudocode - Core Enums
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! These enums represent fundamental ARM architectural concepts used across
//! multiple shared pseudocode functions.

use core::fmt;

/// Exception levels in AArch64
///
/// Stanford pseudocode: `bits(2)` values representing EL0-EL3
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExceptionLevel {
    #[default]
    EL0 = 0,
    EL1 = 1,
    EL2 = 2,
    EL3 = 3,
}

impl ExceptionLevel {
    /// Maximum exception level
    pub const MAX: ExceptionLevel = ExceptionLevel::EL3;

    /// Check if this exception level exists in the implementation
    pub fn is_implemented(self) -> bool {
        // EL0 and EL1 are always implemented
        // EL2 and EL3 are optional
        matches!(self, ExceptionLevel::EL0 | ExceptionLevel::EL1) || self.has_el()
    }

    /// Check if this EL has the corresponding architectural feature
    fn has_el(self) -> bool {
        // This would be determined by ID_AA64PFR0_EL1 or similar
        // For now, assume EL2 and EL3 are implemented
        true
    }

    /// Convert from bits(2) value
    pub const fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0 => Some(ExceptionLevel::EL0),
            1 => Some(ExceptionLevel::EL1),
            2 => Some(ExceptionLevel::EL2),
            3 => Some(ExceptionLevel::EL3),
            _ => None,
        }
    }

    /// Get the bits(2) representation
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for ExceptionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EL{}", *self as u8)
    }
}

/// Security states
///
/// Stanford pseudocode: `SecurityState` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityState {
    /// Secure state
    Secure,
    /// Non-secure state
    NonSecure,
    /// Realm state (FEAT_RME)
    Realm,
    /// Root state (FEAT_RME)
    Root,
}

impl SecurityState {
    /// Convert from bits(2) value (when FEAT_RME is not implemented)
    pub const fn from_bits_legacy(bits: u8) -> Option<Self> {
        match bits {
            0 => Some(SecurityState::Secure),
            1 => Some(SecurityState::NonSecure),
            _ => None,
        }
    }

    /// Convert from bits(2) value (FEAT_RME)
    pub const fn from_bits_rme(bits: u8) -> Option<Self> {
        match bits {
            0b00 => Some(SecurityState::Secure),
            0b01 => Some(SecurityState::NonSecure),
            0b10 => Some(SecurityState::Realm),
            0b11 => Some(SecurityState::Root),
            _ => None,
        }
    }
}

impl fmt::Display for SecurityState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityState::Secure => write!(f, "Secure"),
            SecurityState::NonSecure => write!(f, "NonSecure"),
            SecurityState::Realm => write!(f, "Realm"),
            SecurityState::Root => write!(f, "Root"),
        }
    }
}

/// ARM architecture condition codes
///
/// Stanford pseudocode: `Condition` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    /// Equal (Z == 1)
    EQ = 0b0000,
    /// Not equal (Z == 0)
    NE = 0b0001,
    /// Carry set (C == 1) / Unsigned >=
    CS = 0b0010,
    /// Carry clear (C == 0) / Unsigned <
    CC = 0b0011,
    /// Minus / Negative (N == 1)
    MI = 0b0100,
    /// Plus / Positive or zero (N == 0)
    PL = 0b0101,
    /// Overflow (V == 1)
    VS = 0b0110,
    /// No overflow (V == 0)
    VC = 0b0111,
    /// Unsigned higher (C == 1 && Z == 0)
    HI = 0b1000,
    /// Unsigned lower or same (C == 0 || Z == 1)
    LS = 0b1001,
    /// Signed greater than or equal (N == V)
    GE = 0b1010,
    /// Signed less than (N != V)
    LT = 0b1011,
    /// Signed greater than (Z == 0 && N == V)
    GT = 0b1100,
    /// Signed less than or equal (Z == 1 || N != V)
    LE = 0b1101,
    /// Always (unconditional)
    AL = 0b1110,
    /// Never (unconditional, but encodes as 0b1111)
    NV = 0b1111,
}

impl Condition {
    /// Get the 4-bit encoding
    pub const fn encoding(self) -> u8 {
        self as u8
    }

    /// Create from 4-bit encoding
    pub const fn from_encoding(bits: u8) -> Option<Self> {
        match bits {
            0b0000 => Some(Condition::EQ),
            0b0001 => Some(Condition::NE),
            0b0010 => Some(Condition::CS),
            0b0011 => Some(Condition::CC),
            0b0100 => Some(Condition::MI),
            0b0101 => Some(Condition::PL),
            0b0110 => Some(Condition::VS),
            0b0111 => Some(Condition::VC),
            0b1000 => Some(Condition::HI),
            0b1001 => Some(Condition::LS),
            0b1010 => Some(Condition::GE),
            0b1011 => Some(Condition::LT),
            0b1100 => Some(Condition::GT),
            0b1101 => Some(Condition::LE),
            0b1110 => Some(Condition::AL),
            0b1111 => Some(Condition::NV),
            _ => None,
        }
    }

    /// Get the inverse condition
    pub const fn invert(self) -> Self {
        match self {
            Condition::EQ => Condition::NE,
            Condition::NE => Condition::EQ,
            Condition::CS => Condition::CC,
            Condition::CC => Condition::CS,
            Condition::MI => Condition::PL,
            Condition::PL => Condition::MI,
            Condition::VS => Condition::VC,
            Condition::VC => Condition::VS,
            Condition::HI => Condition::LS,
            Condition::LS => Condition::HI,
            Condition::GE => Condition::LT,
            Condition::LT => Condition::GE,
            Condition::GT => Condition::LE,
            Condition::LE => Condition::GT,
            Condition::AL => Condition::NV,
            Condition::NV => Condition::AL,
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::EQ => write!(f, "EQ"),
            Condition::NE => write!(f, "NE"),
            Condition::CS => write!(f, "CS"),
            Condition::CC => write!(f, "CC"),
            Condition::MI => write!(f, "MI"),
            Condition::PL => write!(f, "PL"),
            Condition::VS => write!(f, "VS"),
            Condition::VC => write!(f, "VC"),
            Condition::HI => write!(f, "HI"),
            Condition::LS => write!(f, "LS"),
            Condition::GE => write!(f, "GE"),
            Condition::LT => write!(f, "LT"),
            Condition::GT => write!(f, "GT"),
            Condition::LE => write!(f, "LE"),
            Condition::AL => write!(f, "AL"),
            Condition::NV => write!(f, "NV"),
        }
    }
}

/// PSTATE condition flags (NZCV)
///
/// Stanford pseudocode: 4-bit NZCV flags in PSTATE
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ConditionFlags {
    /// Negative flag (bit 3)
    pub n: bool,
    /// Zero flag (bit 2)
    pub z: bool,
    /// Carry flag (bit 1)
    pub c: bool,
    /// Overflow flag (bit 0)
    pub v: bool,
}

impl ConditionFlags {
    /// Create from individual flags
    pub const fn new(n: bool, z: bool, c: bool, v: bool) -> Self {
        Self { n, z, c, v }
    }

    /// Create from 4-bit NZCV value (N=bit3, Z=bit2, C=bit1, V=bit0)
    pub const fn from_nzcv(bits: u8) -> Self {
        Self {
            n: (bits & 0b1000) != 0,
            z: (bits & 0b0100) != 0,
            c: (bits & 0b0010) != 0,
            v: (bits & 0b0001) != 0,
        }
    }

    /// Get as 4-bit NZCV value
    pub const fn to_nzcv(self) -> u8 {
        ((self.n as u8) << 3) | ((self.z as u8) << 2) | ((self.c as u8) << 1) | (self.v as u8)
    }

    /// Check if condition holds given these flags
    pub fn condition_holds(self, cond: Condition) -> bool {
        match cond {
            Condition::EQ => self.z,
            Condition::NE => !self.z,
            Condition::CS => self.c,
            Condition::CC => !self.c,
            Condition::MI => self.n,
            Condition::PL => !self.n,
            Condition::VS => self.v,
            Condition::VC => !self.v,
            Condition::HI => self.c && !self.z,
            Condition::LS => !self.c || self.z,
            Condition::GE => self.n == self.v,
            Condition::LT => self.n != self.v,
            Condition::GT => !self.z && (self.n == self.v),
            Condition::LE => self.z || (self.n != self.v),
            Condition::AL => true,
            Condition::NV => false,
        }
    }
}

/// Translation regimes
///
/// Stanford pseudocode: `Regime` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationRegime {
    /// EL1&0 translation regime (secure or non-secure)
    EL10,
    /// EL2 translation regime
    EL2,
    /// EL2&0 translation regime
    EL20,
    /// EL3 translation regime
    EL3,
    /// Secure EL1&0 regime (AArch32 only)
    EL30,
}

/// Cache operation types
///
/// Stanford pseudocode: `CacheOp` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheOp {
    Invalidate,
    Clean,
    CleanInvalidate,
}

/// Cache operation scope
///
/// Stanford pseudocode: `CacheOpScope` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheOpScope {
    /// Point of Unification
    PoU,
    /// Point of Coherency
    PoC,
    /// Set/Way
    SetWay,
}

/// Shareability domains
///
/// Stanford pseudocode: `Shareability` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Shareability {
    #[default]
    NonShareable,
    InnerShareable,
    OuterShareable,
}

/// Memory types
///
/// Stanford pseudocode: `MemType` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemType {
    Normal,
    Device,
}

/// Device memory types
///
/// Stanford pseudocode: `DeviceType` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    nGnRnE,
    nGnRE,
    nGRE,
    GRE,
}

/// Memory attributes
///
/// Stanford pseudocode: `MemAttrHints` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemAttr {
    NonCacheable,
    WriteThrough,
    WriteBack,
    WriteBackTransient, // FEAT_TWE
}

/// Fault status codes
///
/// Stanford pseudocode: `Fault` status codes
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultStatus {
    None,
    Alignment,
    Debug,
    AccessFlag,
    Permission,
    Translation,
    AddressSize,
    SyncExternal,
    AsyncExternal,
    SyncExternalOnWalk,
    AsyncExternalOnWalk,
    TLBConflict,
    UnsupportedExclusive,
    Domain, // AArch32 only
}

impl fmt::Display for FaultStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FaultStatus::None => write!(f, "Fault_None"),
            FaultStatus::Alignment => write!(f, "Fault_Alignment"),
            FaultStatus::Debug => write!(f, "Fault_Debug"),
            FaultStatus::AccessFlag => write!(f, "Fault_AccessFlag"),
            FaultStatus::Permission => write!(f, "Fault_Permission"),
            FaultStatus::Translation => write!(f, "Fault_Translation"),
            FaultStatus::AddressSize => write!(f, "Fault_AddressSize"),
            FaultStatus::SyncExternal => write!(f, "Fault_SyncExternal"),
            FaultStatus::AsyncExternal => write!(f, "Fault_AsyncExternal"),
            FaultStatus::SyncExternalOnWalk => write!(f, "Fault_SyncExternalOnWalk"),
            FaultStatus::AsyncExternalOnWalk => write!(f, "Fault_AsyncExternalOnWalk"),
            FaultStatus::TLBConflict => write!(f, "Fault_TLBConflict"),
            FaultStatus::UnsupportedExclusive => write!(f, "Fault_UnsupportedExclusive"),
            FaultStatus::Domain => write!(f, "Fault_Domain"),
        }
    }
}

/// Constraint outcomes for CONSTRAINED UNPREDICTABLE behavior
///
/// Stanford pseudocode: `Constraint` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Constraint {
    /// The behavior is disabled (no effect)
    Disabled,
    /// The behavior is UNKNOWN (any valid value)
    Unknown,
    /// The behavior is NONE (use the architectural default)
    None,
    /// Implementation defined choice
    ImpDef,
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Disabled => write!(f, "Constraint_DISABLED"),
            Constraint::Unknown => write!(f, "Constraint_UNKNOWN"),
            Constraint::None => write!(f, "Constraint_NONE"),
            Constraint::ImpDef => write!(f, "Constraint_IMPDEF"),
        }
    }
}

/// Unpredictable behavior types
///
/// Stanford pseudocode: `Unpredictable` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unpredictable {
    VCMATCHDAPA,
    VCMATCHHALF,
    BPMATCHHALF,
    BPMISMATCHHALF,
    BPNOTIMPL,
    BPNOTCTXCMP,
    BPLINKINGDISABLED,
    RESBPTYPE,
    WPMASKANDBAS,
    WPBASCONTIGUOUS,
    WPMASKEDBITS,
    RESWPMASK,
    COUNT_CHAIN,
    // ... many more would be defined
}

impl fmt::Display for Unpredictable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unpredictable::VCMATCHDAPA => write!(f, "Unpredictable_VCMATCHDAPA"),
            Unpredictable::VCMATCHHALF => write!(f, "Unpredictable_VCMATCHHALF"),
            Unpredictable::BPMATCHHALF => write!(f, "Unpredictable_BPMATCHHALF"),
            Unpredictable::BPMISMATCHHALF => write!(f, "Unpredictable_BPMISMATCHHALF"),
            Unpredictable::BPNOTIMPL => write!(f, "Unpredictable_BPNOTIMPL"),
            Unpredictable::BPNOTCTXCMP => write!(f, "Unpredictable_BPNOTCTXCMP"),
            Unpredictable::BPLINKINGDISABLED => write!(f, "Unpredictable_BPLINKINGDISABLED"),
            Unpredictable::RESBPTYPE => write!(f, "Unpredictable_RESBPTYPE"),
            Unpredictable::WPMASKANDBAS => write!(f, "Unpredictable_WPMASKANDBAS"),
            Unpredictable::WPBASCONTIGUOUS => write!(f, "Unpredictable_WPBASCONTIGUOUS"),
            Unpredictable::WPMASKEDBITS => write!(f, "Unpredictable_WPMASKEDBITS"),
            Unpredictable::RESWPMASK => write!(f, "Unpredictable_RESWPMASK"),
            Unpredictable::COUNT_CHAIN => write!(f, "Unpredictable_COUNT_CHAIN"),
        }
    }
}

/// Access types for memory operations
///
/// Stanford pseudocode: `AccessType` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessType {
    IFetch,
    GPR,
    FP,
    ASIMD,
    SVE,
    SME,
    IC,
    DC,
    DCZero,
    AT,
    NV2,
    SPE,
    GCS,
    TRBE,
    GPTW,
    HACDBS,
    HDBSS,
    TTW,
}

/// Endianness
///
/// Stanford pseudocode: `Endianness` type
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}

/// Branch predictor behavior for BRBE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchPredictorBehavior {
    PredictNotTaken,
    PredictTaken,
    Mispredicted,
}

/// PMU event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PMUEventType {
    CpuCycles,
    Chain,
    // ... many more
}

/// Physical memory return status (IMPDEF)
///
/// Stanford pseudocode: `PhysMemRetStatus` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysMemRetStatus {
    /// Successful access
    Success,
    /// Alignment fault
    AlignmentFault,
    /// Access flag fault
    AccessFlagFault,
    /// Permission fault
    PermissionFault,
    /// Translation fault
    TranslationFault,
    /// Address size fault
    AddressSizeFault,
    /// Synchronous external abort
    SyncExternalAbort,
    /// Asynchronous external abort
    AsyncExternalAbort,
    /// Synchronous external abort on translation table walk
    SyncExternalAbortOnWalk,
    /// Asynchronous external abort on translation table walk
    AsyncExternalAbortOnWalk,
    /// TLB conflict
    TLBConflict,
    /// Unsupported exclusive access
    UnsupportedExclusive,
    /// Implementation-defined error
    ImpDefError,
}

/// Memory operation type
///
/// Stanford pseudocode: `MemOp` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemOp {
    /// Load operation
    LOAD,
    /// Store operation
    STORE,
    /// Prefetch operation
    PREFETCH,
    /// Atomic read-modify-write operation
    ATOMIC,
}

/// Branch type for branch target identification and hinting
///
/// Stanford pseudocode: `BranchType` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchType {
    /// Direct branch (B, B.cond)
    DIR,
    /// Direct call (BL)
    DIRCALL,
    /// Indirect call (BLR)
    INDCALL,
    /// Indirect branch (BR)
    INDIR,
    /// Return (RET)
    RET,
    /// Exception return (ERET)
    ERET,
    /// Exception entry
    EXCEPTION,
    /// Reset
    RESET,
    /// Debug exit
    DBGEXIT,
}

/// Memory atomic operation type
///
/// Stanford pseudocode: `MemAtomicOp` enumeration
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemAtomicOp {
    /// Atomic add (LDADD, STADD)
    ADD,
    /// Atomic bit clear (LDBIC, STBIC)
    BIC,
    /// Compare and swap (CAS, CASA, CASL, CASAL)
    CAS,
    /// Atomic exclusive or (LDEOR, STEOR)
    EOR,
    /// GCS stack save (RCW)
    GCSSS1,
    /// Atomic bit set (LDORR, STORR)
    ORR,
    /// Atomic signed max (LDSMAX, STSMAX)
    SMAX,
    /// Atomic signed min (LDSMIN, STSMIN)
    SMIN,
    /// Swap (SWP, SWPA, SWPL, SWPAL)
    SWP,
    /// Atomic unsigned max (LDUMAX, STUMAX)
    UMAX,
    /// Atomic unsigned min (LDUMIN, STUMIN)
    UMIN,
}