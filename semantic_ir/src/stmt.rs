use crate::expr::Expr;
use crate::types::{BitWidth, Exception, Flag, MemAccess, MemOrdering, MemSize, Reg, SpecialReg, Width};

/// Arm of a Match statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    pub patterns: Vec<MatchPattern>,
    pub guard: Option<Expr>,
    pub body: Vec<Stmt>,
}

/// Pattern for Match arms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchPattern {
    /// Constant pattern (e.g., `0`, `'1'`, `BranchType_DIR`)
    Constant(Expr),
    /// Wildcard pattern (`_`)
    Wildcard,
    /// Variable binding pattern (binds to name)
    Binding(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    WriteReg {
        reg: Reg,
        value: Expr,
    },

    WriteIndexedReg {
        index: Expr,
        value: Expr,
        width: Width,
    },

    WriteSP {
        value: Expr,
        width: Width,
    },

    WriteSpecial {
        reg: SpecialReg,
        value: Expr,
        width: Width,
    },

    WritePredReg {
        reg: u8,
        value: Expr,
        pl: BitWidth,
    },

    WriteScalableVecReg {
        reg: u8,
        value: Expr,
        vl: BitWidth,
    },

    WriteFlag {
        flag: Flag,
        value: Expr,
    },

    MemWrite {
        address: Expr,
        value: Expr,
        size: MemSize,
        access: MemAccess,
        ordering: MemOrdering,
    },

    AtomicLoadEx {
        dest: Reg,
        address: Expr,
        size: MemSize,
        ordering: MemOrdering,
    },

    AtomicStoreEx {
        status: Reg,
        address: Expr,
        value: Expr,
        size: MemSize,
        ordering: MemOrdering,
    },

    AtomicSwap {
        dest: Reg,
        address: Expr,
        value: Expr,
        size: MemSize,
        ordering: MemOrdering,
    },

    AtomicCas {
        address: Expr,
        expected: Reg,
        desired: Reg,
        size: MemSize,
        ordering: MemOrdering,
    },

    AtomicCasPair {
        address: Expr,
        expected_first: Reg,
        expected_second: Reg,
        desired_first: Reg,
        desired_second: Reg,
        size: MemSize,
        ordering: MemOrdering,
    },
    MemRead {
        address: Expr,
        dest: Reg,
        size: MemSize,
        access: MemAccess,
        ordering: MemOrdering,
    },

    Let {
        name: String,
        value: Expr,
    },

    If {
        condition: Expr,
        then_body: Vec<Self>,
        else_body: Vec<Self>,
    },

    Branch {
        target: Expr,
    },

    BranchConditional {
        condition: Expr,
        target: Expr,
    },

    RaiseException(Exception),

    Discard {
        value: Expr,
    },

    /// Assignment to an existing variable (mutating a `Let`-bound name).
    /// Stanford: `Assign { target: Name, value }` or slice/field assign.
    Assign {
        name: String,
        value: Expr,
    },

    /// Slice/field assignment (e.g., `X[n][31:0] = value`, `PSTATE.Z = '1'`).
    AssignPlace {
        place: Expr,  // Place expression (Slice, FieldAccess, Name via ReadReg)
        value: Expr,
    },

    /// Case/Match statement for multi-way branching.
    /// Stanford: `Case { expr, arms: [IrArm { patterns, guard, body }] }`
    Match {
        scrutinee: Expr,
        arms: Vec<MatchArm>,
    },

    /// While loop with iteration limit (for bounded execution).
    /// Stanford: `While { cond, body }` or `For` loops.
    While {
        condition: Expr,
        body: Vec<Self>,
        max_iterations: Option<u32>, // From recurseLimit
    },

    /// Function return.
    Return {
        value: Option<Expr>,
    },
}
