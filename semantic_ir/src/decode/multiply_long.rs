use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_multiply_long(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let op23_21 = (bits >> 21) & 0x7;  // bits 23-21
    let rm = (bits >> 16) & 0x1F;
    let ra = (bits >> 10) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;
    let bit15 = (bits >> 15) & 1;

    // Verify fixed bits: bits 28-24 = 11011 (0x1B)
    if ((bits >> 24) & 0x1F) != 0x1B {
        return Err(DecodeError::UndefinedInstruction);
    }
    // bits 23-21 must be one of the valid patterns: 001, 010, 011, 110
    if op23_21 != 0b001 && op23_21 != 0b010 && op23_21 != 0b011 && op23_21 != 0b110 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = match (op23_21, bit15) {
        (0b001, 0) => "UMULL",
        (0b001, 1) => "UMADDL",
        (0b011, 0) => "SMULL",
        (0b011, 1) => "SMADDL",
        (0b010, 0) => "SMULH",
        (0b110, 0) => "UMULH",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    let input_width = if op23_21 == 0b010 || op23_21 == 0b110 {
        // MULH uses 64-bit inputs
        BitWidth::W64
    } else {
        // MULL/MADDL uses 32-bit inputs
        BitWidth::W32
    };

    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
        match Reg::x(idx) {
            Some(Reg::X(n)) if w == BitWidth::W32 => Ok(Reg::W(n)),
            Some(Reg::Xzr) => {
                if w == BitWidth::W32 {
                    Ok(Reg::Wzr)
                } else {
                    Ok(Reg::Xzr)
                }
            }
            Some(r) => Ok(r),
            None => Err(DecodeError::InvalidRegister),
        }
    }

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: to_reg(rd as u8, BitWidth::W64)?,  // Rd is always 64-bit
        rn: to_reg(rn as u8, input_width)?,  // Rn width depends on instruction
        rm: to_reg(rm as u8, input_width)?,  // Rm width depends on instruction
        shift: ShiftKind::Lsl,
        shift_amount: ra, // store ra (accumulator) here
        imm: 0,
        n: 0,
        width: BitWidth::W64,  // Result is always 64-bit
    })
}