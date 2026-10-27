use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_branch_reg(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let opc = (bits >> 21) & 0x7;  // bits 23-21
    let rn = (bits >> 5) & 0x1F;   // bits 9-5
    
    // Verify fixed bits: bits 31-24 = 11010110 (0xD6), bits 20-10 = 11111000000 (0x7C0), bit 4 = 0
    if ((bits >> 24) & 0xFF) != 0xD6 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 10) & 0x7FF) != 0x7C0 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 4) & 1) != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = match opc {
        0b000 => "BR",
        0b001 => "BLR",
        0b010 => "RET",
        0b100 => "ERET",
        0b101 => "DRPS",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    let width = BitWidth::W64;

    fn to_reg(idx: u8, _w: BitWidth) -> Result<Reg, DecodeError> {
        match Reg::x(idx) {
            Some(Reg::X(n)) => Ok(Reg::X(n)),
            Some(Reg::Xzr) => Ok(Reg::Xzr),
            Some(_) => Err(DecodeError::InvalidRegister),
            None => Err(DecodeError::InvalidRegister),
        }
    }

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: Reg::Xzr,  // Not used for branch register
        rn: to_reg(rn as u8, width)?,
        rm: Reg::Xzr,  // Not used
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: 0,
        n: 0,
        width,
    })
}