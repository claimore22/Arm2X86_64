//! AArch64 instruction decoder.
//!
//! Currently decodes only ADD (shifted register) in both 32-bit and 64-bit forms.
//! All other encodings return DecodeError::UndefinedInstruction.

use crate::expr::{Expr, BinOp};
use crate::instruction::{Instruction, InstructionId};
use crate::primitives::ShiftKind;
use crate::stmt::Stmt;
use crate::types::{BitWidth, Reg, ValueType, Width};
use crate::Provenance;

/// Decode error for unsupported or invalid encodings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Reserved or undefined encoding.
    UndefinedInstruction,
    /// Invalid shift type for ADD (shifted register).
    InvalidShift,
    /// Register index out of range (must be 0-30).
    InvalidRegister,
}

/// Decoded representation of an AArch64 instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub mnemonic: String,
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub shift: ShiftKind,
    pub shift_amount: u32,
    pub width: BitWidth,
}

/// Decode a 32-bit AArch64 instruction.
///
/// Currently only decodes ADD (shifted register) in both 32-bit and 64-bit forms.
/// Returns DecodeError::UndefinedInstruction for all other encodings.
pub fn decode_instruction(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op0 = (bits >> 29) & 3;
    let shift = (bits >> 22) & 3;
    let rm = (bits >> 16) & 0x1F;
    let imm6 = (bits >> 10) & 0x3F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    // ADD (shifted register): sf op0=00 bit28=1 bit27=0 bit26=0 bit25=1 bit24=0 shift Rm imm6 Rn Rd
    if op0 != 0b00 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 28) & 1) != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 27) & 1) != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 26) & 1) != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 25) & 1) != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 24) & 1) != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let shift_kind = match shift {
        0 => ShiftKind::Lsl,
        _ => return Err(DecodeError::InvalidShift),
    };

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    let rd_reg = match Reg::x(rd as u8) {
        Some(Reg::X(n)) if width == BitWidth::W32 => Reg::W(n),
        Some(Reg::Xzr) => Reg::Wzr,
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) if width == BitWidth::W32 => Reg::W(n),
        Some(Reg::Xzr) => Reg::Wzr,
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };
    let rm_reg = match Reg::x(rm as u8) {
        Some(Reg::X(n)) if width == BitWidth::W32 => Reg::W(n),
        Some(Reg::Xzr) => Reg::Wzr,
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };

    Ok(DecodedInstruction {
        mnemonic: "ADD".to_string(),
        rd: rd_reg,
        rn: rn_reg,
        rm: rm_reg,
        shift: shift_kind,
        shift_amount: imm6,
        width,
    })
}

/// Lower a decoded instruction into semantic IR.
pub fn lower_to_ir(decoded: &DecodedInstruction) -> Instruction {
    let ty = ValueType::Bits(Width::Fixed(decoded.width));

    let rn_expr = Expr::ReadReg {
        reg: decoded.rn,
        width: decoded.width,
    };

    let rm_expr = Expr::ReadReg {
        reg: decoded.rm,
        width: decoded.width,
    };

    let shift_amount = Expr::Const {
        value: decoded.shift_amount as u128,
        ty: decoded.width,
    };

    let shifted = Expr::Shift {
        kind: decoded.shift,
        value: Box::new(rm_expr),
        amount: Box::new(shift_amount),
        ty: decoded.width,
    };

    let result = Expr::Binary {
        op: BinOp::Add,
        left: Box::new(rn_expr),
        right: Box::new(shifted),
        ty,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: decoded.rd,
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: decoded.mnemonic.clone(),
            encoding: Some(format!(
                "ADD (shifted register) {}",
                decoded.width.bits()
            )),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "arm64_md_clean/add_shifted.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
