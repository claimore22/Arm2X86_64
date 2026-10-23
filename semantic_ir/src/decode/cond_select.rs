use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_cond_select(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op0 = (bits >> 29) & 3; // 00 csel/csinc, 10 csinv/csneg
    let rm = (bits >> 16) & 0x1F;
    let cond = (bits >> 12) & 0xF;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;
    let bit10 = (bits >> 10) & 1;

    let mnemonic = if op0 == 0 {
        if bit10 == 0 { "CSEL" } else { "CSINC" }
    } else if op0 == 2 {
        if bit10 == 0 { "CSINV" } else { "CSNEG" }
    } else {
        return Err(DecodeError::UndefinedInstruction);
    };

    // bit28? top bytes 0x9A/0xDA: bit28=1? 0x9A=10011010 bit28=1. check bit28=1
    if ((bits >> 28) & 1) != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
        match Reg::x(idx) {
            Some(Reg::X(n)) if w == BitWidth::W32 => Ok(Reg::W(n)),
            Some(Reg::Xzr) => {
                if w == BitWidth::W32 { Ok(Reg::Wzr) } else { Ok(Reg::Xzr) }
            }
            Some(r) => Ok(r),
            None => Err(DecodeError::InvalidRegister),
        }
    }
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: to_reg(rd as u8, width)?,
        rn: to_reg(rn as u8, width)?,
        rm: to_reg(rm as u8, width)?,
        shift: ShiftKind::Lsl,
        shift_amount: 0,
        imm: cond,
        n: 0,
        width,
    })
}
