use crate::expr::Expr;
use crate::types::BitWidth;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShiftKind {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtendKind {
    Uxtb,
    Uxth,
    Uxtw,
    Uxtx,
    Sxtb,
    Sxth,
    Sxtw,
    Sxtx,
}

/// Vector unary operations for element-wise mapping (SIMD&FP).
/// These operate per-element on a vector register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorUnaryOp {
    // Integer
    Abs,
    Neg,
    Not,
    Clz,
    Cls,
    PopCnt,
    Cnt,        // Population count per byte (CNT)
    Rev16,      // Reverse elements in 16-bit halfwords
    Rev32,      // Reverse elements in 32-bit words
    Rev64,      // Reverse elements in 64-bit doublewords
    RBIT,       // Reverse bit order

    // Floating-point
    FPAbs,
    FPNeg,
    FPSqrt,
    FRecipEstimate,
    FRecipSqrtEstimate,
    FRintN,     // Round to nearest, ties to even
    FRintP,     // Round toward +inf
    FRintM,     // Round toward -inf
    FRintZ,     // Round toward zero
    FRintA,     // Round to nearest, ties away
    FRintX,     // Round to integral exact
    FRintI,     // Round using current rounding mode
    FRint32X,   // Round to 32-bit integer, current mode
    FRint32Z,   // Round to 32-bit integer, toward zero
    FRint64X,   // Round to 64-bit integer, current mode
    FRint64Z,   // Round to 64-bit integer, toward zero

    // FP conversion
    FCVTZS,     // FP to signed int, toward zero
    FCVTZU,     // FP to unsigned int, toward zero
    FCVTNS,     // FP to signed int, nearest even
    FCVTNU,     // FP to unsigned int, nearest even
    FCVTMS,     // FP to signed int, toward -inf
    FCVTMU,     // FP to unsigned int, toward -inf
    FCVTPS,     // FP to signed int, toward +inf
    FCVTPU,     // FP to unsigned int, toward +inf
    FCVTAS,     // FP to signed int, ties away
    FCVTAU,     // FP to unsigned int, ties away

    // BFloat16
    BF16CVT,    // FP32 to BF16

    // FP8
    FP8CVT,     // FP8 to higher precision

    // Integer comparison with zero (vector unary)
    CMEQZero,   // Compare equal to zero
    CMGEZero,   // Compare signed >= zero
    CMGTZero,   // Compare signed > zero
    CMLEZero,   // Compare signed <= zero
    CMLTZero,   // Compare signed < zero
}

/// Vector binary operations for element-wise mapping (SIMD&FP).
/// These operate per-element on two vector registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorBinOp {
    // Integer arithmetic
    Add,
    Sub,
    Mul,
    UDiv,
    SDiv,
    UMod,
    SMod,

    // Saturating arithmetic
    SQAdd,
    UQAdd,
    SQSub,
    UQSub,
    SQMul,
    UQMul,
    SQDMul,
    SQDMulH,

    // Halving arithmetic
    UHAdd,
    SHAdd,
    UHSub,
    SHSub,

    // Bitwise
    And,
    Orr,
    Xor,
    Bic,        // Bit clear
    Eor3,       // Three-way XOR
    Bsl,        // Bitwise select
    Bit,        // Bitwise insert if true
    Bif,        // Bitwise insert if false

    // Comparison (produce all-ones/all-zeros per element)
    CMEQ,       // Equal
    CMGE,       // Signed >=
    CMGT,       // Signed >
    CMHI,       // Unsigned >
    CMHS,       // Unsigned >=
    CMLE,       // Signed <= (vs zero)
    CMLT,       // Signed < (vs zero)
    CMTST,      // Test bits nonzero

    // FP arithmetic
    FPAdd,
    FPSub,
    FPMul,
    FPDiv,
    FPMulAdd,   // Fused multiply-add
    FPMulSub,   // Fused multiply-subtract

    // FP comparison
    FCMEQ,      // FP equal
    FCMGE,      // FP >=
    FCMGT,      // FP >
    FCMLE,      // FP <= (vs zero)
    FCMLT,      // FP < (vs zero
    FACGE,      // FP abs >=
    FACGT,      // FP abs >

    // FP max/min
    FPMax,
    FPMin,
    FPMaxNum,
    FPMinNum,

    // FP pairwise
    FAddP,
    FMaxP,
    FMinP,
    FMaxNMP,
    FMinNMP,

    // Integer max/min
    SMax,
    SMin,
    UMax,
    UMin,

    // Dot products
    SDot,
    UDot,
    USDot,
    SUDot,

    // BFloat16
    BFDot,      // BFloat16 dot product
    BFMulAdd,   // BFloat16 multiply-add

    // FP8
    FP8Dot,     // FP8 dot product

    // Widening (long) operations - produce 2*esize from esize
    SAddL,
    UAddL,
    SSubL,
    USubL,
    SMulL,
    UMulL,
    SMLAL,
    UMLAL,
    SMLSL,
    UMLSL,
    SQDMLAL,
    SQDMLSL,
    FMLAL,
    FMLSL,
    BFMLAL,

    // Narrowing operations - produce esize/2 from esize
    XTN,
    SQXTN,
    UQXTN,
    SHRN,
    RSHRN,
    SQSHRN,
    UQSHRN,
    SQRSHRN,
    FCVTN,

    // AddHN/SubHN
    AddHN,
    RAddHN,
    SubHN,
    RSubHN,
}

/// Narrowing operations for vector instructions that reduce element size.
/// These take wider source elements and produce narrower result elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NarrowingOp {
    // AddHN/SubHN: add/sub then shift right by esize (take high half)
    AddHN,
    RAddHN,
    SubHN,
    RSubHN,
    // Other narrowing ops
    XTN,
    SQXTN,
    UQXTN,
    SHRN,
    RSHRN,
    SQSHRN,
    UQSHRN,
    SQRSHRN,
    FCVTN,
    FCVTXN,
    SQXTUN,
    UQXTUN,
}

/// Vector reduce operations (horizontal reduction across vector).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorReduceOp {
    AddV,       // Add across vector
    SAddLV,     // Signed add long across vector
    UAddLV,     // Unsigned add long across vector
    MaxV,       // Max across vector
    MinV,       // Min across vector
    MaxNV,      // MaxNum across vector
    MinNV,      // MinNum across vector
}

/// Vector dot product operations (for BF16, FP8, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorDotOp {
    BFDot,      // BFloat16 dot product to FP32
    BF16MulAdd, // BF16 multiply-add to FP32
    FP8Dot,     // FP8 dot product to FP16/FP32
    SDot,       // Signed integer dot product
    UDot,       // Unsigned integer dot product
}

/// Vector pairwise operations (adjacent elements).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorPairwiseOp {
    AddP,
    FAddP,
    MaxP,
    MinP,
    MaxNMP,
    MinNMP,
    FMaxP,
    FMinP,
    FMaxNMP,
    FMinNMP,
}

/// Vector permutation operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorPermOp {
    Ext,        // Extract from pair
    Tbl,        // Table lookup
    Tbx,        // Table lookup extension
    Trn1,       // Transpose primary
    Trn2,       // Transpose secondary
    Zip1,       // Zip primary
    Zip2,       // Zip secondary
    Uzp1,       // Unzip primary
    Uzp2,       // Unzip secondary
    Rev16,
    Rev32,
    Rev64,
    Dup,        // Duplicate element
    Ins,        // Insert element
    
    // AES operations
    AESInvShiftRows,
    AESInvSubBytes,
    AESShiftRows,
    AESSubBytes,
    AESMixColumns,
    AESInvMixColumns,
}

/// Vector deinterleave operations (for FP8 to BF16 conversion with downscaling).
/// These take a single vector and deinterleave it into two destination vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorDeinterleaveOp {
    /// FP8 to BF16 conversion with downscaling (BF1CVTL/BF2CVTL)
    /// Takes pairs of 8-bit elements, converts each to BF16 with scaling,
    /// and writes to two destination vectors (deinterleaved).
    FP8ToBF16Deinterleave,
    FP8ToBF16Deinterleave2,
}

/// Vector interleave operations (for FP8 to BF16 conversion).
/// Takes two source vectors and interleaves their elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorInterleaveOp {
    /// FP8 to BF16 conversion with interleaving (BFCVTN)
    FP8ToBF16Interleave,
}

/// Atomic read-modify-write operation kind (FEAT_LSE and friends).
///
/// Mirrors the Stanford `MemAtomicOp_*` enumeration: the named operation is
/// applied atomically to the memory location using the register operand, and
/// the original memory value is returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicRMWOp {
    Add,
    Bic,
    Eor,
    Orr,
    Smax,
    Smin,
    Umax,
    Umin,
    Swp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    Eq,
    Ne,
    Cs,
    Cc,
    Mi,
    Pl,
    Vs,
    Vc,
    Hi,
    Ls,
    Ge,
    Lt,
    Gt,
    Le,
    Al,
    Nv,
}

/// Scalar floating-point binary operations.
/// These operate on single FP elements (not vectors).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarFPBinOp {
    FPAdd,
    FPSub,
    FPMul,
    FPDiv,
    FPMulAdd,
    FPMulSub,
    FPMax,
    FPMin,
    FPMaxNum,
    FPMinNum,
}

/// Pointer-authentication-code key kind for PAC computation (FEAT_PAuth).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacKind {
    /// AddPACIA / PACIA* (instruction address, key A)
    IA,
    /// AddPACIB / PACIB* (instruction address, key B)
    IB,
    /// AddPACDA / PACDA* (data address, key A)
    DA,
    /// AddPACDB / PACDB* (data address, key B)
    DB,
    /// AddPACGA / PACGA (generic key)
    GA,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionHolds {
    pub condition: Condition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddWithCarry {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub carry: Box<Expr>,
    pub width: BitWidth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendReg {
    pub register: u8,
    pub kind: ExtendKind,
    pub shift: Box<Expr>,
    pub width: BitWidth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredicationMode {
    Merging,
    Zeroing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEPredicateOp {
    And,
    Or,
    Orn,
    Xor,
    Nand,
    Nor,
    Not,
    Ands,
    Orrs,
    Eors,
    Nands,
    Nors,
    Sel,
    Eq,
    Ge,
    Gt,
    Ne,
    Lt,
    Le,
    Unordered,
    Brka,
    Brkas,
    Brkb,
    Brkbs,
    Brkn,
    Brkns,
    Brkpa,
    Brkpas,
    Brkpb,
    Brkpbs,
    Ptest,
    Pfirst,
    Ptrue,
    Ptrues,
    Pfalse,
    Pnext,
    Ctermeq,
    Ctermne,
    Cntp,
    Cntb,
    Cntd,
    Cnth,
    Cntw,
    Decp,
    Incp,
    Setffr,
    Rdffr,
    Rdffrs,
    Pext,
    Psel,
    Punpkh,
    Punpkl,
    Punpklo,
    Rev,
    // Floating-point absolute compare
    Facge,
    Facgt,
    Faclt,
    Faceq,
    Facle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEVectorUnaryOp {
    Abs,
    Neg,
    Not,
    Clz,
    Cls,
    Cnt,
    Revb,
    Revh,
    Revw,
    Revd,
    Rbit,
    Fabs,
    Fneg,
    Frintn,
    Frintp,
    Frintm,
    Frintz,
    Frinta,
    Frintx,
    Frinti,
    FRint32X,
    FRint32Z,
    FRint64X,
    FRint64Z,
    Fsqrt,
    Fscale,
    Frsqrte,
    Frecpe,
    FRecpX,
    Fexpa,
    Flogb,
    FDup,
    Dup,
    Id,
    Asr,
    Asrd,
    Lsl,
    Lsr,
    Fcvtzu,
    Fcvtzs,
    Fcvt,
    Fcvtn,
    Fcvtnt,
    Fcvtx,
    Fcvtxn,
    Fcvtl,
    Fcvtlt,
    Fcvtnb,
    Bfcvt,
    Bfcvtn,
    Bfcvtnt,
    Bfloat16Cvt,
    Bfloat16Cvtl,
    Bfloat16Cvtlt,
    Scvtf,
    Sqrt,
    Rsqrt,
    Exp,
    Log,
    Sin,
    Cos,
    Rev,
    SQAbs,
    SQNeg,
    Bfconvertfp8,
    Sunpkhi,
    Sunpklo,
    Ursqrte,
    SQSHRN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEVectorBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Udiv,
    Sdiv,
    SDivR,
    UDivR,
    Umod,
    Smod,
    Min,
    Max,
    MinNum,
    MaxNum,
    And,
    Or,
    Xor,
    Bic,
    Eor3,
    Bsl,
    Lsl,
    Lsr,
    Asr,
    Asrd,
    Fadd,
    Fsub,
    Fsubr,
    Fmul,
    Fdiv,
    Frecps,
    Frsqrts,
    Fmla,
    Fmls,
    Fmad,
    Fmsb,
    Fnmla,
    Fnmul,
    Fnmadd,
    Fnmsub,
    Fcadd,
    Fcmla,
    Fcmlalb,
    Fcmlalt,
    Fcmlslb,
    Fcmlslt,
    Bfloat16Add,
    Bfloat16Sub,
    Bfloat16Mul,
    Bfloat16Fma,
    Bfloat16Fms,
    Bfloat16Mla,
    Bfloat16Mls,
    Bfloat16Mlalb,
    Bfloat16Mlalt,
    Bfloat16Mlslb,
    Bfloat16Mlslt,
    Bfloat16Dot,
    Bfloat16Mmla,
    Bfloat16Max,
    Bfloat16Min,
    Bfloat16MaxNum,
    Bfloat16MinNum,
    Fdot,
    Fdot4,
    Fmlla,
    Fmmla,
    Fcmlal,
    Fcmlsl,
    Fcvtn2,
    Fcvtl2,
    Fcvtxn2,
    Fcvtx2,
    Fadda,
    Fmaxa,
    Fmina,
    Fmaxnma,
    Fminnma,
    Fmaxv,
    Fminv,
    Fmaxnmv,
    Fminnmv,
    Fmaxp,
    Fminp,
    Fmaxnmp,
    Fminnmp,
    Faddv,
    Faddqv,
    Fmaxqv,
    Fminqv,
    Fmaxnmqv,
    Fminnmqv,
    Faddp,
    SMulh,
    UMulh,
    SQAdd,
    UQAdd,
    SQSub,
    UQSub,
    SMax,
    SMin,
    UMax,
    UMin,
    UHAdd,
    UHSub,
    Fabd,
    FMaxNum,
    FMinNum,
    FMul,
    FDiv,
    FMax,
    FMin,
    FMaxP,
    FMinP,
    FScale,
    FMulX,
    FPMulAdd,
    FPMulSub,
    FDivR,
    FCAdd,
    FDup,
    FExpA,
    FLogB,
    Ftsmul,
    Ftssel,
    Bdep,
    Bext,
    FAMax,
    FAMin,
    SUQAdd,
    Urshl,
    Urshrl,
    Sqshlu,
    Sqdmulh,
    Usqadd,
    Ursh,
    PMul,
    RAddHN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEVectorReduceOp {
    AddV,
    SAddV,
    UAddV,
    MaxV,
    MinV,
    MaxNV,
    MinNV,
    AndV,
    OrV,
    XorV,
    AndQV,
    OrQV,
    XorQV,
    EorQV,
    SMaxV,
    UMaxV,
    SMinV,
    UMinV,
    SMaxQV,
    UMaxQV,
    SMinQV,
    UMinQV,
    SAddQV,
    UAddQV,
    FAddV,
    FAddQV,
    AddQV,
    FMaxV,
    FMinV,
    FMaxNV,
    FMinNV,
    FMaxQV,
    FMinQV,
    FMaxNQV,
    FMinNQV,
    FMaxNMV,
    FMinNMV,
    FMaxNMQV,
    FMinNMQV,
    FAddA,  // FADDA: strictly-ordered reduction with scalar initial value
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEVectorTernaryOp {
    Bfloat16Fma,
    Bfloat16Fms,
    Bfloat16Mla,
    Bfloat16Mls,
    Fmla,
    Fmls,
    Fmad,
    Fmsb,
    Fnmla,
    Fnmul,
    Fnmadd,
    Fnmsub,
    Fcadd,
    Fcmla,
    Fcmlalb,
    Fcmlalt,
    Fcmlslb,
    Fcmlslt,
    Ftmad,
    Fmul,
    Adclb,
    Adclt,
    Bcax,
    FClamp,
    Bfloat16Scale,
    Fcvtnt,
    Fcvtnb,
    Sqrshl,
    Sqrshlr,
    Sqshl,
    Sqshlr,
    Sqshlu,
    Sqshrnb,
    Sqshrnt,
    Sqshrunb,
    Sqshrunt,
    Sli,
    Sri,
    Shadd,
    Hsub,
    Hsubr,
    Urshl,
    Urshrl,
    Sqdmulh,
    Sqrdmlah,
    Sqrdmlsh,
    Sshllb,
    Sshllt,
    Ushllb,
    Ushllt,
    Uqrshl,
    Uqrshlr,
    Uqshl,
    Uqshlr,
    Uqshrnb,
    Uqshrnt,
    Ursh,
    Uqsubr,
    Uqadd,
    Uqsub,
    Umullb,
    Umullt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEVectorPairwiseOp {
    FMaxP,
    FMinP,
    FMaxNMP,
    FMinNMP,
    FAddP,
    AddP,
    MaxP,
    MinP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SVEGatherScatterOp {
    ContiguousLoad,
    ContiguousStore,
    GatherLoad,
    ScatterStore,
    FirstFaultLoad,
    NonFaultLoad,
    NonTemporalLoad,
    NonTemporalStore,
    ReplicateLoad,
    ReplicateStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)] // architecture operation names use underscores
pub enum SVESpecialOp {
    Index,
    Dup,
    Dupm,
    Dupq,
    Cpy,
    Fmov,
    Insr,
    Ext,
    Extq,
    Trn1,
    Trn2,
    Zip1,
    Zip2,
    Uzp1,
    Uzp2,
    Compact,
    Expand,
    Sel,
    Movprfx,
    LastA,
    LastB,
    LastA_Scalar,
    LastB_Scalar,
    Clasta,
    Clastb,
    Histcnt,
    Histseg,
    Match,
    Nmatch,
    Rdvl,
    BFClamp,
    FirstP,
    DecodePredCount,
    CountPred,
    CounterToPredicate,
    AESInvSubBytes,
    AESInvShiftRows,
    AESInvMixColumns,
    AESSubBytes,
    AESShiftRows,
    AESMixColumns,
    FAddQVSeg,  // FADDQV: segment-wise reduction to 128-bit
    LastP,      // LASTP: scalar index of last true predicate element
    Rdffr,      // RDFFR: read first-fault register
    Rdffrs,     // RDFFRS: read first-fault register with condition flags
    Setffr,     // SETFFR: set first-fault register to all true
    Wrffr,      // WRFFR: write first-fault register
    Xar,        // XAR: bitwise exclusive-OR and rotate right by immediate
    Uunpkhi,    // UUNPKHI: unsigned unpack high half
    Uunpklo,    // UUNPKLO: unsigned unpack low half
    Whilelt,    // WHILELT: while incrementing signed scalar less than scalar
    Whilele,    // WHILELE: while incrementing signed scalar less than or equal to scalar
    Whilelo,    // WHILELO: while incrementing unsigned scalar lower than scalar
    Whilels,    // WHILELS: while incrementing unsigned scalar lower than or same as scalar
    Whilegt,    // WHILEGT: while decrementing signed scalar greater than scalar
    Whilege,    // WHILEGE: while decrementing signed scalar greater than or equal to scalar
    Whilehi,    // WHILEHI: while decrementing unsigned scalar higher than scalar
    Whilehs,    // WHILEHS: while decrementing unsigned scalar higher than or same as scalar
    Whilerw,    // WHILERW: while free of read-after-write conflicts
    Whilewr,    // WHILEWR: while free of write-after-read/write conflicts
    Cadd,       // CADD: complex integer add
    // New instructions from remaining checklist
    Cdot,           // CDOT: complex integer dot product
    Srshl,          // SRSHL: signed rounding shift left
    Ld1shGatherImm, // LD1SH: gather load signed halfwords (immediate index)
    Ld1shGather,    // LD1SH: gather load signed halfwords (scalar index)
    Ld1swGatherImm, // LD1SW: gather load signed words (immediate index)
    Ld1swGather,    // LD1SW: gather load signed words (scalar index)
    Ldnt1bImm,      // LDNT1B: contiguous load non-temporal bytes (immediate index)
    Ldnt1bScalar,   // LDNT1B: contiguous load non-temporal bytes (scalar index)
    Ldnt1hImm,      // LDNT1H: contiguous load non-temporal halfwords (immediate index)
    Ldnt1hScalar,   // LDNT1H: contiguous load non-temporal halfwords (scalar index)
    Ldnt1wImm,      // LDNT1W: contiguous load non-temporal words (immediate index)
    Ldnt1wScalar,   // LDNT1W: contiguous load non-temporal words (scalar index)
    St1bImm,        // ST1B: contiguous store bytes (immediate index)
    St1bScalar,     // ST1B: contiguous store bytes (scalar index)
    St1hImm,        // ST1H: contiguous store halfwords (immediate index)
    St1hScalar,     // ST1H: contiguous store halfwords (scalar index)
    St1wImm,        // ST1W: contiguous store words (immediate index)
    St1wScalar,     // ST1W: contiguous store words (scalar index)
    Stnt1bImm,      // STNT1B: contiguous store non-temporal bytes (immediate index)
    Stnt1bScalar,   // STNT1B: contiguous store non-temporal bytes (scalar index)
    Stnt1hImm,      // STNT1H: contiguous store non-temporal halfwords (immediate index)
    Stnt1hScalar,   // STNT1H: contiguous store non-temporal halfwords (scalar index)
    Stnt1wImm,      // STNT1W: contiguous store non-temporal words (immediate index)
    Stnt1wScalar,   // STNT1W: contiguous store non-temporal words (scalar index)
    PrfbImm,        // PRFB: contiguous prefetch bytes (immediate index)
    PrfbScalar,     // PRFB: contiguous prefetch bytes (scalar index)
    PrfhImm,        // PRFH: contiguous prefetch halfwords (immediate index)
    PrfhScalar,     // PRFH: contiguous prefetch halfwords (scalar index)
    PrfwImm,        // PRFW: contiguous prefetch words (immediate index)
    PrfwScalar,     // PRFW: contiguous prefetch words (scalar index)
    Uclamp,         // UCLAMP: unsigned clamp to minimum/maximum
    BfmulIndexed,   // BFMUL (indexed): BFloat16 multiply by indexed element
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    AddWithCarry(AddWithCarry),
    ExtendReg(ExtendReg),
}
