use crate::primitives::{Condition, ExtendKind, PredicationMode, ScalarFPBinOp, ShiftKind, SVEGatherScatterOp, SVEPredicateOp, SVEVectorBinOp, SVEVectorReduceOp, SVEVectorTernaryOp, SVEVectorUnaryOp, SVEVectorPairwiseOp, SVESpecialOp, VectorBinOp, VectorUnaryOp, VectorReduceOp, VectorPairwiseOp, VectorPermOp, VectorDeinterleaveOp, VectorDotOp, VectorInterleaveOp, NarrowingOp};
use crate::types::{BitWidth, Flag, MemOrdering, MemSize, Reg, SpecialReg, ValueType, Width};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Mul,

    Shl,
    Lsr,
    Asr,

    Eq,
    Ne,

    LtSigned,
    LtUnsigned,
    LeSigned,
    LeUnsigned,
    GtSigned,
    GtUnsigned,
    GeSigned,
    GeUnsigned,

    // Saturating arithmetic
    SQAdd,
    UQAdd,
    SQSub,
    UQSub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Not,
    Neg,
    PopCnt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const {
        value: u128,
        ty: BitWidth,
    },

    Bool(bool),

    ReadReg {
        reg: Reg,
        width: BitWidth,
    },

    ReadIndexedReg {
        index: Box<Self>,
        width: BitWidth,
    },

    ReadSP {
        width: BitWidth,
    },

    ReadSpecial {
        reg: SpecialReg,
        width: BitWidth,
    },

    SystemReg {
        reg: u8,
    },

    ReadFlag {
        flag: Flag,
    },

    Binary {
        op: BinOp,
        left: Box<Self>,
        right: Box<Self>,
        ty: ValueType,
    },

    Unary {
        op: UnaryOp,
        value: Box<Self>,
        ty: ValueType,
    },

    /// Scalar floating-point binary operation.
    /// Operates on single FP elements of a specific width (16, 32, or 64 bits).
    /// Examples: FADD (scalar), FSUB (scalar), FMUL (scalar), FDIV (scalar)
    ScalarFPBinary {
        op: ScalarFPBinOp,
        left: Box<Self>,
        right: Box<Self>,
        ty: BitWidth,  // Element size (W16, W32, W64)
    },

    Shift {
        kind: ShiftKind,
        value: Box<Self>,
        amount: Box<Self>,
        ty: BitWidth,
    },

    Extend {
        kind: ExtendKind,
        value: Box<Self>,
        to: BitWidth,
    },

    ExtendReg {
        register: Box<Self>,
        kind: ExtendKind,
        shift: Box<Self>,
        to: Width,
    },

    ZeroExtend {
        value: Box<Self>,
        to: BitWidth,
    },

    SignExtend {
        value: Box<Self>,
        to: BitWidth,
    },

    BitTest {
        value: Box<Self>,
        bit: u16,
    },

    Extract {
        value: Box<Self>,
        lsb: u16,
        width: BitWidth,
    },

    Concat {
        high: Box<Self>,
        low: Box<Self>,
        width: BitWidth,
    },

    Replicate {
        value: Box<Self>,
        count: u16,
        element_width: BitWidth,
    },

    /// Vector element-wise unary map: apply unary op to each element of a vector.
    /// Models: `for e = 0 to elements-1 do result[e*esize] = op(operand[e*esize]) end`
    /// Examples: FABS, FNEG, CLZ, CLS, CNT, REV16/32/64, FRSQRTE, FRINTN, FCVTZS, etc.
    VectorMap {
        op: VectorUnaryOp,
        operand: Box<Self>,
        datasize: BitWidth,   // Total vector size (64 or 128)
        esize: BitWidth,      // Element size (8, 16, 32, 64)
    },

    /// Vector element-wise binary map: apply binary op to corresponding elements.
    /// Models: `for e = 0 to elements-1 do result[e*esize] = op(left[e*esize], right[e*esize]) end`
    /// Examples: ADD, SUB, MUL, FMUL, FADD, AND, ORR, CMEQ, FCMGE, FPMULADD, etc.
    VectorMap2 {
        op: VectorBinOp,
        left: Box<Self>,
        right: Box<Self>,
        datasize: BitWidth,
        esize: BitWidth,
    },

    /// Vector horizontal reduction: reduce across all elements.
    /// Models: `result = op(operand[0], operand[1], ..., operand[elements-1])`
    /// Examples: ADDV, SADDLV, UADDLV, MAXV, MINV
    VectorReduce {
        op: VectorReduceOp,
        operand: Box<Self>,
        datasize: BitWidth,
        esize: BitWidth,
    },

    /// Vector pairwise operation: operate on adjacent element pairs.
    /// Models: `for e = 0 to elements/2-1 do result[e*esize] = op(operand[e*2], operand[e*2+1]) end`
    /// Examples: ADDP (scalar: same size), ADDP (vector: half size), FADDP, MAXP, MINP
    VectorPairwise {
        op: VectorPairwiseOp,
        operand: Box<Self>,
        operand_datasize: BitWidth,  // Input vector size
        result_datasize: BitWidth,   // Output vector size (half for vector ADDP, same for scalar)
        esize: BitWidth,
    },

    /// Vector narrowing operation: takes wider source elements, operates, then narrows.
    /// Models: `for e = 0 to elements-1 do result[e*esize] = op(left[e*2*esize], right[e*2*esize]) >> esize end`
    /// Examples: ADDHN, ADDHN2, SUBHN, SUBHN2, RADDHN, RSUBHN, XTN, SHRN, FCVTN, etc.
    VectorNarrowing {
        op: NarrowingOp,
        left: Box<Self>,
        right: Box<Self>,
        datasize: BitWidth,   // Source datasize (wider, e.g., 128-bit)
        esize: BitWidth,      // Source element size (wider, e.g., 64-bit for 2*esize)
        round: bool,          // Round vs truncate
        part: u8,             // Destination part (0=lower half, 1=upper half for *2 variants)
    },

    /// Vector permutation operations.
    /// Examples: EXT, TBL, TBX, TRN1, TRN2, ZIP1, ZIP2, UZP1, UZP2, REV16/32/64, DUP, INS
    VectorPerm {
        op: VectorPermOp,
        operands: Vec<Self>,  // 1-4 operands depending on operation
        datasize: BitWidth,
        esize: BitWidth,
        imm: Option<u16>,     // For EXT position, DUP lane, etc.
    },

    /// Vector deinterleave operations (for FP8 to BF16 conversion with downscaling).
    /// Examples: BF1CVTL, BF2CVTL, BF1CVTL2, BF2CVTL2
    VectorDeinterleave {
        op: VectorDeinterleaveOp,
        operand: Box<Self>,   // Source vector (VL bits)
        datasize: BitWidth,   // Source vector size (VL)
        esize: BitWidth,      // Source element size (8-bit FP8)
        scale: Box<Self>,     // FPMR.LSCALE or FPMR.LSCALE2 for downscaling
        f8_format: Box<Self>, // FPMR.F8S1 or FPMR.F8S2 for encoding format
        fpcr: Box<Self>,      // FPCR for rounding
        fpmr: Box<Self>,      // FPMR for scale and format selection
    },

    /// Vector interleave operations (for FP8 to BF16 conversion).
    /// Takes two source vectors and interleaves their elements.
    /// Examples: BFCVTN (FP8 to BF16 conversion with interleaving)
    VectorInterleave {
        op: VectorInterleaveOp,
        left: Box<Self>,      // First source vector
        right: Box<Self>,     // Second source vector
        datasize: BitWidth,   // Source vector size (VL)
        esize: BitWidth,      // Source element size (8-bit FP8)
        result_esize: BitWidth, // Result element size (16-bit BF16)
    },

    /// Vector dot product operations.
    /// Examples: BFDOT, BFMLAL, BFMLALT, FP8Dot
    VectorDot {
        op: VectorDotOp,
        a: Box<Self>,           // First operand (pairs of elements)
        b: Box<Self>,           // Second operand (pairs of elements)
        addend: Box<Self>,      // Accumulator (single-precision)
        datasize: BitWidth,     // Vector size (e.g., 128-bit)
        esize: BitWidth,        // Result element size (32-bit for BFDOT)
        fpcr: Box<Self>,        // FPCR for rounding
        fpmr: Option<Box<Self>>, // FPMR for FP8 (optional)
        broadcast_index: Option<u8>, // For by-element variant (BFDOT, BFMLAL by element)
    },

    Condition(Condition),

    ConditionHolds {
        condition: Condition,
    },

    /// MTE logical-address-tag selection.
    ///
    /// Models `AArch64_ChooseNonExcludedTagOrZero(start_tag, tag_offset,
    /// GCR_EL1.Exclude)`: `start_tag` and `tag_offset` are 4-bit values,
    /// `gcr` is the whole 64-bit GCR_EL1 (the Exclude field is selected by
    /// the lowering/interpreter per the spec, not pinned here). Result is
    /// always a 4-bit tag, like `BitTest` always yields `Bool`.
    /// First required by ADDG (MTE "add with tag").
    ChooseTag {
        start_tag: Box<Self>,
        tag_offset: Box<Self>,
        gcr: Box<Self>,
    },

    /// MTE random tag selection (FEAT_MTE).
    ///
    /// Models `AArch64_ChooseTagOrZero(exclude)`: returns an
    /// implementation-chosen 4-bit tag not in the given exclude set
    /// (`exclude` is the 64-bit source register; the lowering ORs in
    /// GCR_EL1.Exclude), or zero if all tags are excluded.
    /// First required by IRG.
    ChooseTagOrZero {
        exclude: Box<Self>,
    },

    /// MTE allocation tag load (FEAT_MTE).
    ///
    /// Models `AArch64_MemTag[address]`: reads the 4-bit allocation tag
    /// attached to the tag granule containing `address` (64-bit).
    /// First required by LDG; reused by LDGM.
    MemTagRead {
        address: Box<Self>,
    },

    /// Checked-pointer result adjustment (FEAT_CPA).
    ///
    /// Models `PointerAddCheck(result, base)`: if the most significant 8
    /// bits of `value` differ from those of `base`, the architecture
    /// modifies the result so it is likely non-canonical as an address
    /// (the exact modification is implementation-defined, hence opaque
    /// here rather than composed from Extract/Select). Both operands and
    /// the result are 64 bits. First required by ADDPT.
    PointerAddCheck { value: Box<Self>, base: Box<Self> },

    /// System maintenance operation (SYS family).
    ///
    /// Models `AArch64_SysInstr(op0, op1, crn, crm, op2, t)` operating on
    /// the 64-bit `address` value: cache/TLB/translation/prediction effects
    /// selected by the fields. Field widths follow the SYS decode
    /// (op0: 2, op1: 3, crn: 4, crm: 4, op2: 3). Statement-level only: use
    /// with `Stmt::Discard`, like `MemRead`. Has no scalar `ValueType`
    /// (same precedent as `AddWithCarry`). First required by SYS; reused
    /// by all SYS aliases (APAS, AT, BRB, ...).
    SysOp {
        op0: Box<Self>,
        op1: Box<Self>,
        crn: Box<Self>,
        crm: Box<Self>,
        op2: Box<Self>,
        address: Box<Self>,
    },

    /// System maintenance operation with register result (SYSL family).
    ///
    /// Models `AArch64_SysInstrWithResult(op0, op1, crn, crm, op2, t)`:
    /// same field widths as `SysOp`, but returns a 64-bit result that the
    /// instruction writes to Xt. First required by SYSL aliases
    /// (GCSPOPM, GCSSS2).
    SysLOp {
        op0: Box<Self>,
        op1: Box<Self>,
        crn: Box<Self>,
        crm: Box<Self>,
        op2: Box<Self>,
        address: Box<Self>,
    },

    /// Pointer Authentication: authenticate data address (key A).
    ///
    /// Models `AuthDA(address, modifier, auth_combined)`: returns the
    /// authenticated address with upper bits restored on success. On
    /// failure the architecture faults (handled by interpreter/lowering).
    /// `address` and `modifier` are 64-bit; `auth_combined` is boolean.
    /// Result is 64-bit. First required by AUTDA/AUTDZA (FEAT_PAuth).
    AuthDA {
        address: Box<Self>,
        modifier: Box<Self>,
        auth_combined: Box<Self>,
    },

    /// Pointer Authentication: authenticate instruction address (key A/B).
    ///
    /// Models `AuthIA(address, modifier, auth_combined)` / `AuthIB(...)`
    /// for instruction addresses. Same signature as `AuthDA`. First
    /// required by AUTIA/AUTIB families.
    AuthIA {
        address: Box<Self>,
        modifier: Box<Self>,
        auth_combined: Box<Self>,
    },
    AuthIB {
        address: Box<Self>,
        modifier: Box<Self>,
        auth_combined: Box<Self>,
    },

    /// Pointer Authentication: authenticate instruction address (key A) with
    /// two modifiers (FEAT_PAuth_LR).
    ///
    /// Models `AuthIA2(address, modifier1, modifier2, auth_combined)`.
    /// Required by AUTIA1716/AUTIASP when PACM=1. First required by
    /// AUTIA family with LR extension.
    AuthIA2 {
        address: Box<Self>,
        modifier1: Box<Self>,
        modifier2: Box<Self>,
        auth_combined: Box<Self>,
    },

    /// Pointer Authentication: authenticate instruction address (key B) with
    /// two modifiers (FEAT_PAuth_LR).
    ///
    /// Models `AuthIB2(address, modifier1, modifier2, auth_combined)`.
    /// Required by AUTIB1716/AUTIBSP when PACM=1. First required by
    /// AUTIB family with LR extension.
    AuthIB2 {
        address: Box<Self>,
        modifier1: Box<Self>,
        modifier2: Box<Self>,
        auth_combined: Box<Self>,
    },

    /// Pointer Authentication: authenticate data address (key B).
    ///
    /// Models `AuthDB(address, modifier, auth_combined)`. Same signature
    /// as `AuthDA` but with key B. First required by AUTDB/AUTDZB.
    AuthDB {
        address: Box<Self>,
        modifier: Box<Self>,
        auth_combined: Box<Self>,
    },

    /// Pointer Authentication: compute and insert a PAC (FEAT_PAuth).
    ///
    /// Models `AddPACI{A,B}/AddPACD{A,B}/AddPACGA(address, modifier)`: returns
    /// the 64-bit value with the pointer authentication code inserted. Opaque
    /// (the QARMA cipher is not composed from IR ops), same precedent as the
    /// Auth* nodes. First required by PACIA/PACIB/PACDA/PACDB/PACGA families.
    AddPAC {
        kind: crate::primitives::PacKind,
        address: Box<Self>,
        modifier: Box<Self>,
    },

    /// PAC with a second modifier (FEAT_PAuth_LR, PSTATE.PACM == 1).
    ///
    /// Models `AddPACI{A,B}2(address, modifier1, modifier2)`. First required
    /// by PACIA1716/PACIASP etc. when PACM=1.
    AddPAC2 {
        kind: crate::primitives::PacKind,
        address: Box<Self>,
        modifier1: Box<Self>,
        modifier2: Box<Self>,
    },

    AddWithCarry {
        left: Box<Self>,
        right: Box<Self>,
        carry: Box<Self>,
        width: BitWidth,
    },

    MemRead {
        address: Box<Self>,
        size: MemSize,
    },

    /// Atomic compare-and-swap memory operation.
    ///
    /// Models `MemAtomic(address, comparevalue, newvalue, accdesc)` with
    /// `MemAtomicOp_CAS`: atomically compares the value at `address` with
    /// `expected`, and if equal, stores `new_value` to `address` with the
    /// specified `ordering`. Returns the original value loaded from `address`
    /// (width determined by `size`).
    /// First required by CAS, CASA, CASAL, CASL (FEAT_LSE).
    MemAtomic {
        address: Box<Self>,
        expected: Box<Self>,
        new_value: Box<Self>,
        size: MemSize,
        ordering: MemOrdering,
    },

    /// Atomic read-modify-write memory operation (FEAT_LSE).
    ///
    /// Models `MemAtomic(address, comparevalue:ARBITRARY, newvalue, accdesc)`
    /// for non-CAS `MemAtomicOp_*`: atomically applies `op` to the memory
    /// location at `address` using `value`, and returns the value originally
    /// loaded from `address` (width determined by `size`).
    /// First required by LDADD/LDADDA/LDADDAL/LDADDL; reused by the
    /// LDCLR/LDEOR/LDSET/LDSMAX/LDSMIN/LDUMAX/LDUMIN/SWP families.
    MemAtomicRMW {
        op: crate::primitives::AtomicRMWOp,
        address: Box<Self>,
        value: Box<Self>,
        size: MemSize,
        ordering: MemOrdering,
    },

    Select {
        condition: Box<Self>,
        when_true: Box<Self>,
        when_false: Box<Self>,
        ty: ValueType,
    },

    /// If-then-else expression
    If {
        cond: Box<Self>,
        then_branch: Box<Self>,
        else_branch: Box<Self>,
    },

    /// Function call expression
    Call {
        func: String,
        args: Vec<Self>,
    },

    TupleElement {
        value: Box<Self>,
        index: u8,
        ty: ValueType,
    },

    /// Feature check operation (FEAT_CHK).
    ///
    /// Models `AArch64_ChkFeat(value)`: an opaque architectural function that
    /// checks feature status and returns a transformed 64-bit value.
    /// First required by CHKFEAT.
    ChkFeat {
        value: Box<Self>,
    },

    /// CRC checksum operation (FEAT_CRC32).
    ///
    /// Models
    /// `BitReverse{32}(Poly32Mod2{32+size}(BitReverse{32}(acc)::Zeros{size} XOR BitReverse{size}(val)::Zeros{32}, poly))`:
    /// opaque because `Poly32Mod2`/`BitReverse` have no composing IR ops.
    /// `acc` is 32-bit, `value` is `data_width` bits (8/16/32/64), result is
    /// 32-bit. `crc32c == false` selects polynomial 0x04C11DB7 (CRC32);
    /// `true` selects 0x1EDC6F41 (CRC32C).
    /// First required by CRC32B/CRC32H/CRC32W/CRC32X and the CRC32C family.
    Crc32 {
        acc: Box<Self>,
        value: Box<Self>,
        data_width: BitWidth,
        crc32c: bool,
    },

    /// SVE predicate register read.
    /// Models `P(g)[PL]` where PL = VL/8 bits.
    ReadPredReg {
        reg: u8,      // P register number (0-15)
        pl: BitWidth, // Predicate length (VL/8)
    },

    /// SVE scalable vector register read.
    /// Models `Z(n)[VL]` where VL is variable (128, 256, 512, ... 2048).
    ReadScalableVecReg {
        reg: u8,      // Z register number (0-31)
        vl: BitWidth, // Vector length (VL)
    },

    /// SVE predicate register write.
    WritePredReg {
        reg: u8,
        value: Box<Self>,
        pl: BitWidth,
    },

    /// SVE scalable vector register write.
    WriteScalableVecReg {
        reg: u8,
        value: Box<Self>,
        vl: BitWidth,
    },

    /// SVE predicate element-wise unary operation.
    /// Models: for each active predicate element, apply unary op.
    /// Examples: NOT, CNT, PTRUE, PFALSE, PFIRST, PNEXT
    PredicateMap {
        op: SVEPredicateOp,
        operand: Box<Self>,
        pl: BitWidth,
        merging: bool,
    },

    /// SVE scalable vector element-wise unary operation with predication.
    /// Models: for each active element (per predicate), apply unary op.
    /// Inactive elements: merging keeps old value, zeroing sets to zero.
    /// Examples: FABS, FNEG, FSQRT, CLZ, CLS, RBIT, ABS, NEG
    ScalableVectorMap {
        op: SVEVectorUnaryOp,
        predicate: Box<Self>,   // Predicate mask P(g)
        operand: Box<Self>,     // Source Z(n)
        destination: Box<Self>, // Destination Z(d) for merging
        vl: BitWidth,
        esize: BitWidth,
        predication: PredicationMode,
    },

    /// SVE scalable vector element-wise binary operation with predication.
    /// Models: for each active element, apply binary op to corresponding elements.
    /// Inactive elements: merging keeps destination value, zeroing sets to zero.
    /// Examples: FADD, FSUB, FMUL, FDIV, ADD, SUB, MUL, AND, ORR, EOR
    ScalableVectorMap2 {
        op: SVEVectorBinOp,
        predicate: Box<Self>,   // Predicate mask P(g)
        left: Box<Self>,        // First source Z(n)
        right: Box<Self>,       // Second source Z(m)
        destination: Box<Self>, // Destination Z(d) for merging
        vl: BitWidth,
        esize: BitWidth,
        predication: PredicationMode,
    },
    
    /// SVE scalable vector element-wise ternary operation with predication.
    /// Models: for each active element, apply ternary op (e.g., fused multiply-add).
    /// Inactive elements: merging keeps destination value, zeroing sets to zero.
    /// Examples: BFMLA, BFMUL_ADD, FMAD
    ScalableVectorMap3 {
        op: SVEVectorTernaryOp,
        predicate: Box<Self>,    // Predicate mask P(g)
        left: Box<Self>,         // First source Z(n)
        middle: Box<Self>,       // Second source Z(m)
        right: Box<Self>,        // Third source/addend Z(da)
        destination: Box<Self>,  // Destination Z(da) for merging
        vl: BitWidth,
        esize: BitWidth,
        predication: PredicationMode,
    },

    /// SVE predicate reduction operation.
    /// Models: reduce predicate elements to scalar/flag.
    /// Examples: PTEST, ANDV, ORV, XORV, CNTP
    PredicateReduce {
        op: SVEPredicateOp,
        operand: Box<Self>,
        pl: BitWidth,
    },

    /// SVE scalable vector horizontal reduction with predication.
    /// Models: reduce active elements across vector to scalar.
    /// Inactive elements treated as identity (0 for add, max for max, etc.).
    /// Examples: UADDV, SADDV, FADDV, FMAXV, FMINV, ANDV, ORV, EORV
    /// For FADDA (strictly-ordered reduction with scalar initial value), includes optional initial_value.
    ScalableVectorReduce {
        op: SVEVectorReduceOp,
        predicate: Box<Self>, // Predicate mask P(g)
        operand: Box<Self>,   // Source Z(n)
        vl: BitWidth,
        esize: BitWidth,
        initial_value: Option<Box<Self>>, // Optional scalar initial value (for FADDA)
    },

    /// SVE scalable vector predicated pairwise operation.
    /// Models: for each active element pair, combine adjacent elements from two vectors.
    /// For even e: operand1[e] and operand1[e+1]; For odd e: operand2[e-1] and operand2[e].
    /// Results interleaved into destination.
    /// Inactive elements: merging keeps destination, zeroing sets to zero.
    /// Examples: FMAXP, FMINP, FMAXNMP, FMINNMP
    ScalableVectorPairwise {
        op: SVEVectorPairwiseOp,
        predicate: Box<Self>,    // Predicate mask P(g)
        left: Box<Self>,         // First source Z(n)
        right: Box<Self>,        // Second source Z(m)
        destination: Box<Self>,  // Destination Z(d) for merging
        vl: BitWidth,
        esize: BitWidth,
        predication: PredicationMode,
    },

    /// SVE gather/scatter load/store operations.
    /// Models predicated memory operations with vector indices.
    SVEGatherScatter {
        op: SVEGatherScatterOp,
        predicate: Box<Self>,   // Predicate mask P(g)
        base: Box<Self>,        // Base address (X(n) or SP)
        index: Box<Self>,       // Vector index Z(m) for gather/scatter
        destination: Box<Self>, // Destination Z(t) for loads, or source for stores
        vl: BitWidth,
        esize: BitWidth,        // Element size of vector
        msize: BitWidth,        // Memory access size (8, 16, 32, 64)
        scale: u8,              // Scale factor (0, 1, 2, 3 for shift by 0,1,2,3)
        offs_unsigned: bool,    // Whether index is unsigned
        data_unsigned: bool,    // Whether data is unsigned (for sign/zero extend)
        first_fault: bool,      // First-faulting behavior
        non_fault: bool,        // Non-faulting behavior
        non_temporal: bool,     // Non-temporal hint
        contiguous: bool,       // Contiguous vs gather
        tag_checked: bool,      // MTE tag checking
    },

    /// SVE special operations (index generation, permutations, etc.)
    SVESpecial {
        op: SVESpecialOp,
        predicate: Option<Box<Self>>, // Optional predicate for predicated ops
        operands: Vec<Box<Self>>,     // Variable number of operands
        vl: BitWidth,
        esize: Option<BitWidth>,      // Element size when applicable
        imm: Option<u16>,             // Immediate value when applicable
    },

    /// SVE predicate binary operation (element-wise on predicates).
    /// Examples: AND, ORR, EOR, NAND, NOR on predicates
    PredicateMap2 {
        op: SVEPredicateOp,
        left: Box<Self>,
        right: Box<Self>,
        pl: BitWidth,
        merging: bool,
    },

    /// Field access on a struct/global (e.g., PSTATE.Z, SCTLR_ELx.EE)
    /// Used pervasively in Stanford pseudocode for architectural globals.
    FieldAccess {
        base: Box<Self>,
        field: String,
    },

    /// Enum literal value (e.g., BranchType_DIR, Decode_OK, Fault_None)
    /// Required for branch types, fault statuses, decode results.
    EnumLit {
        enum_name: String,
        variant: String,
    },

    /// Slice range for bit extraction (lo..=hi inclusive)
    /// More expressive than Extract for multi-part slices.
    Slice {
        value: Box<Self>,
        lsb: u16,
        msb: u16,
    },

    /// Generic type-parameterized primitive application
    /// Models UInt<N>(x), SInt<N>(x), ZeroExtend<N>(x), SignExtend<N>(x), etc.
    GenericPrim {
        name: String,
        type_args: Vec<BitWidth>,
        args: Vec<Self>,
    },
}
