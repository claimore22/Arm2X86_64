use crate::stmt::Stmt;
use crate::types::ValueType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstructionId {
    pub mnemonic: String,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register { name: String, ty: ValueType },

    Immediate { name: String, ty: ValueType },

    Memory { name: String },

    Condition { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeSpec {
    pub statements: Vec<DecodeStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeStmt {
    Let { name: String, value: DecodeExpr },

    Require { condition: DecodeExpr },

    Undefined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeExpr {
    Identifier(String),

    Immediate(i64),

    UInt(String),

    In { value: Box<Self>, values: Vec<i64> },

    Eq { left: Box<Self>, right: Box<Self> },

    Not { value: Box<Self> },

    ShiftLeft { value: Box<Self>, amount: u8 },

    Add { left: Box<Self>, right: Box<Self> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub id: InstructionId,
    pub operands: Vec<Operand>,
    pub required_features: Vec<String>,
    pub decode: Option<DecodeSpec>,
    pub semantics: Vec<Stmt>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source_path: String,
    pub source_sha256: String,
    pub clean_sha256: String,
}
