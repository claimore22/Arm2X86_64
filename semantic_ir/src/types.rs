use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitWidth(u16);

impl BitWidth {
    pub const W1: Self = Self(1);
    pub const W2: Self = Self(2);
    pub const W3: Self = Self(3);
    pub const W4: Self = Self(4);
    pub const W6: Self = Self(6);
    pub const W8: Self = Self(8);
    pub const W16: Self = Self(16);
    pub const W32: Self = Self(32);
    pub const W64: Self = Self(64);
    pub const W128: Self = Self(128);
    pub const W256: Self = Self(256);
    pub const W512: Self = Self(512);
    pub const W1024: Self = Self(1024);
    pub const W2048: Self = Self(2048);

    pub const fn bits(self) -> u16 {
        self.0
    }

    pub const fn new(bits: u16) -> Option<Self> {
        if bits == 0 || bits > 2048 {
            None
        } else {
            Some(Self(bits))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Width {
    Fixed(BitWidth),
    InstructionDataSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Bits(Width),
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg {
    X(u8),
    W(u8),
    V(u8),
}

impl Reg {
    pub const fn x(index: u8) -> Option<Self> {
        if index < 31 {
            Some(Self::X(index))
        } else {
            None
        }
    }

    pub const fn w(index: u8) -> Option<Self> {
        if index < 31 {
            Some(Self::W(index))
        } else {
            None
        }
    }

    pub const fn v(index: u8) -> Option<Self> {
        if index < 32 {
            Some(Self::V(index))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialReg {
    Xzr,
    Wzr,
    Sp,
    Wsp,
    Pc,
    GcrEl1,
    /// Program counter (64-bit). Used in AUTIASPPC for PAC address computation.
    Pc64,
    /// Exception Link Register (ELR_ELx)
    ElrElx,
    /// Saved Program Status Register (SPSR_ELx)
    SpsrElx,
    /// Floating-point Control Register
    Fpcr,
    /// Floating-point Mode Register (for FP8 control)
    Fpmr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    N,
    Z,
    C,
    V,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemSize {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
    Bits128,
}

impl MemSize {
    pub const fn width(self) -> BitWidth {
        match self {
            Self::Bits8 => BitWidth::W8,
            Self::Bits16 => BitWidth::W16,
            Self::Bits32 => BitWidth::W32,
            Self::Bits64 => BitWidth::W64,
            Self::Bits128 => BitWidth::W128,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemAccess {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemOrdering {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Exception {
    #[error("undefined instruction")]
    UndefinedInstruction,

    #[error("unpredictable architectural behavior")]
    Unpredictable,

    #[error("unknown architectural state")]
    Unknown,

    #[error("permission fault")]
    PermissionFault,

    #[error("alignment fault")]
    AlignmentFault,

    #[error("translation fault")]
    TranslationFault,

    #[error("data abort")]
    DataAbort,

    #[error("instruction abort")]
    InstructionAbort,

    #[error("software exception")]
    SoftwareException,
}
