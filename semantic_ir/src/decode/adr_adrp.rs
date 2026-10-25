use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_adr_adrp(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let a = (bits >> 31) & 1;
    let _immlo = (bits >> 29) & 3;
    let immhi = (bits >> 5) & 0x7FFFF;
    let rd = bits & 0x1F;

    // ADR: bits 31-24 = 0001 0000 (0x10)
    // ADRP: bits 31-24 = 1001 0000 (0x90)
    let top8 = (bits >> 24) & 0xFF;
    if top8 != 0x10 && top8 != 0x90 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = if top8 == 0x10 { "ADR" } else { "ADRP" };
    let width = BitWidth::W64;

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
        rn: Reg::Xzr,  // Not used for ADR/ADRP
        rm: Reg::Xzr,  // Not used
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: immhi as u32,  // Store immhi as immediate
        n: 0,
        width,
    })
}