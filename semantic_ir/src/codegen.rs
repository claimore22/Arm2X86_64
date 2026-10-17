//! Minimal x86-64 code generator for ARM64 instructions.
//!
//! Currently supports only ADD (shifted register) in 64-bit form.
//! Emits raw x86-64 machine code bytes that can be executed natively.
//!
//! Register mapping:
//!   ARM64 X0 -> x86-64 RAX
//!   ARM64 X1 -> x86-64 RCX
//!   ARM64 X2 -> x86-64 RDX
//!   ...etc.

use crate::expr::Expr;
use crate::instruction::Instruction;
use crate::primitives::ShiftKind;
use crate::stmt::Stmt;
use crate::types::{BitWidth, Reg, ValueType, Width};

/// Error type for code generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodegenError {
    /// Instruction mnemonic not supported by this codegen.
    UnsupportedMnemonic(String),
    /// Instruction operand not supported.
    UnsupportedOperand(String),
    /// Instruction width not supported.
    UnsupportedWidth(BitWidth),
}

/// x86-64 register encoding (low 3 bits of ModR/M).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum X86Reg {
    RAX = 0,
    RCX = 1,
    RDX = 2,
    RBX = 3,
    RSP = 4,
    RBP = 5,
    RSI = 6,
    RDI = 7,
}

/// Map ARM64 X register to x86-64 register for the bootstrap.
fn arm_reg_to_x86(reg: &Reg) -> Result<X86Reg, CodegenError> {
    match reg {
        Reg::X(0) => Ok(X86Reg::RAX),
        Reg::X(1) => Ok(X86Reg::RCX),
        Reg::X(2) => Ok(X86Reg::RDX),
        Reg::X(3) => Ok(X86Reg::RBX),
        Reg::X(4) => Ok(X86Reg::RSP),
        Reg::X(5) => Ok(X86Reg::RBP),
        Reg::X(6) => Ok(X86Reg::RSI),
        Reg::X(7) => Ok(X86Reg::RDI),
        Reg::X(_) => Err(CodegenError::UnsupportedOperand(format!(
            "X register {:?} not yet mapped to x86-64", reg
        ))),
        Reg::W(_) => Err(CodegenError::UnsupportedOperand(format!(
            "W register not supported in 64-bit codegen: {:?}", reg
        ))),
        Reg::V(_) => Err(CodegenError::UnsupportedOperand(format!(
            "V register not supported in x86-64 codegen: {:?}", reg
        ))),
        Reg::Xzr => Err(CodegenError::UnsupportedOperand(
            "XZR not yet supported in codegen".to_string()
        )),
        Reg::Wzr => Err(CodegenError::UnsupportedOperand(
            "WZR not yet supported in x86-64 codegen".to_string()
        )),
    }
}

/// Emit x86-64 machine code for ADD (shifted register).
///
/// Currently supports: ADD Xd, Xn, Xm, LSL #0 (64-bit only).
/// Encoding: 48 89 C8 (mov rax, rcx) + 48 01 D8 (add rax, rdx) + C3 (ret)
/// where X0->RAX, X1->RCX, X2->RDX.
fn emit_add_shifted_register(decoded: &crate::decode::DecodedInstruction) -> Result<Vec<u8>, CodegenError> {
    // Check width FIRST, before trying to convert registers (which would fail for W regs)
    if decoded.width != BitWidth::W64 {
        return Err(CodegenError::UnsupportedWidth(decoded.width));
    }

    let rd = x86_reg_from_decoded(&decoded.rd)?;
    let rn = x86_reg_from_decoded(&decoded.rn)?;
    let rm = x86_reg_from_decoded(&decoded.rm)?;

    let mut bytes: Vec<u8> = Vec::new();

    // mov rax, rcx  -> RAX = X1
    // REX.W = 0x48, opcode = 0x89, modrm = 11 001 000 = 0xC8
    bytes.push(0x48);
    bytes.push(0x89);
    bytes.push(0xC8 | ((rn as u8) << 3) | rd as u8);

    // add rax, rdx -> RAX = RAX + X2
    // REX.W = 0x48, opcode = 0x01, modrm = 11 010 000 = 0xD8
    bytes.push(0x48);
    bytes.push(0x01);
    bytes.push(0xC8 | ((rm as u8) << 3) | rd as u8);

    // ret near
    bytes.push(0xC3);

    Ok(bytes)
}

/// Lower a decoded instruction to x86-64 machine code bytes.
pub fn lower_to_x86_64(decoded: &crate::decode::DecodedInstruction) -> Result<Vec<u8>, CodegenError> {
    if decoded.mnemonic != "ADD" {
        return Err(CodegenError::UnsupportedMnemonic(decoded.mnemonic.clone()));
    }
    emit_add_shifted_register(decoded)
}

/// Convert a DecodedInstruction register to x86-64 register.
fn x86_reg_from_decoded(reg: &Reg) -> Result<X86Reg, CodegenError> {
    arm_reg_to_x86(reg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_shifted_register_codegen() {
        // ADD X0, X1, X2, LSL #0 -> 0x92020020
        let bits: u32 = 0x92020020;
        let decoded = crate::decode::decode_instruction(bits)
            .expect("ADD X0, X1, X2, LSL #0 should decode");

        let bytes = lower_to_x86_64(&decoded).expect("ADD should lower to x86-64");

        // Expected bytes: 48 89 C8 (mov rax, rcx) + 48 01 D8 (add rax, rdx) + C3 (ret)
        assert_eq!(bytes, vec![0x48, 0x89, 0xC8, 0x48, 0x01, 0xD8, 0xC3]);
    }

    #[test]
    fn test_add_shifted_register_32bit_rejected() {
        // ADD W0, W1, W2, LSL #0 -> 0x12020020 (32-bit)
        let bits: u32 = 0x12020020;
        let decoded = crate::decode::decode_instruction(bits)
            .expect("ADD W0, W1, W2, LSL #0 should decode");

        let result = lower_to_x86_64(&decoded);
        assert!(matches!(result, Err(CodegenError::UnsupportedWidth(_))));
    }

    #[test]
    fn test_non_add_instruction_rejected() {
        // Create a fake InstructionId with non-ADD mnemonic
        let decoded = crate::decode::DecodedInstruction {
            mnemonic: "SUB".to_string(),
            rd: Reg::X(0),
            rn: Reg::X(1),
            rm: Reg::X(2),
            shift: crate::primitives::ShiftKind::Lsl,
            shift_amount: 0,
            width: BitWidth::W64,
        };

        let result = lower_to_x86_64(&decoded);
        assert!(matches!(result, Err(CodegenError::UnsupportedMnemonic(_))));
    }
}
