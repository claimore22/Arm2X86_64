use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_hint(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    // HINT instructions: 0xD50320xx where xx = (op << 5) | 0x1F
    // Fixed bits: bits 31-8 = 0xD50320
    if (bits >> 8) != 0xD50320 {
        return Err(DecodeError::UndefinedInstruction);
    }
    
    let hint_code = bits & 0xFF;
    // The hint code must have bits 4-0 = 0x1F (NOP form)
    if (hint_code & 0x1F) != 0x1F {
        return Err(DecodeError::UndefinedInstruction);
    }
    
    let op = (hint_code >> 5) & 0x7;
    
    let mnemonic = match op {
        0b000 => "NOP",
        0b001 => "YIELD",
        0b010 => "WFE",
        0b011 => "WFI",
        0b100 => "SEV",
        0b101 => "SEVL",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: Reg::Xzr,
        rn: Reg::Xzr,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: 0,
        n: 0,
        width: BitWidth::W64,
    })
}