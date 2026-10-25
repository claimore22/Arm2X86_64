use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_unary(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let opc = (bits >> 29) & 3;
    let n = (bits >> 28) & 1;
    let op2 = (bits >> 14) & 0x1F;  // bits 18-14
    let rn = (bits >> 5) & 0x1F;    // bits 9-5
    let rd = bits & 0x1F;           // bits 4-0

    // Verify fixed bits: bits 27-23 = 10101 (0x15), bits 22-19 = 1100 (0xC)
    if ((bits >> 23) & 0x1F) != 0b10101 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 19) & 0xF) != 0xC {
        return Err(DecodeError::UndefinedInstruction);
    }

    // opc must be 10 (0b10) for these instructions
    if opc != 0b10 {
        return Err(DecodeError::UndefinedInstruction);
    }
    // N must be 1 (these instructions require N=1)
    if n != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = match op2 {
        0b00000 => "REV",
        0b00001 => "REV16",
        0b00010 => "REV32",
        0b00011 => "REV64",
        0b00100 => "CLZ",
        0b00101 => "CLS",
        0b00110 => "RBIT",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    // REV64 only valid in 64-bit mode
    if mnemonic == "REV64" && width == BitWidth::W32 {
        return Err(DecodeError::InvalidRegister);
    }

    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
        match Reg::x(idx) {
            Some(Reg::X(n)) if w == BitWidth::W32 => Ok(Reg::W(n)),
            Some(Reg::X(n)) => Ok(Reg::X(n)),
            Some(Reg::Xzr) => {
                if w == BitWidth::W32 {
                    Ok(Reg::Wzr)
                } else {
                    Ok(Reg::Xzr)
                }
            }
            Some(_) => Err(DecodeError::InvalidRegister),
            None => Err(DecodeError::InvalidRegister),
        }
    }

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: to_reg(rd as u8, width)?,
        rn: to_reg(rn as u8, width)?,
        rm: Reg::Xzr, // Not used for unary ops
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: op2,
        n: 0,
        width,
    })
}