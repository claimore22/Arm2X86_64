use crate::types::{BitWidth, Reg};
use crate::primitives::ShiftKind;
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_add_sub(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op0 = (bits >> 29) & 3;
    let shift = (bits >> 22) & 3;
    let rm = (bits >> 16) & 0x1F;
    let imm6 = (bits >> 10) & 0x3F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    let mnemonic = if op0 == 0b00 {
        "ADD"
    } else if op0 == 0b10 {
        "SUB"
    } else {
        return Err(DecodeError::UndefinedInstruction);
    };
    if ((bits >> 28) & 1) != 1
        || ((bits >> 27) & 1) != 0
        || ((bits >> 26) & 1) != 0
        || ((bits >> 25) & 1) != 1
        || ((bits >> 24) & 1) != 0
    {
        return Err(DecodeError::UndefinedInstruction);
    }

    let shift_kind = match shift {
        0 => ShiftKind::Lsl,
        1 => ShiftKind::Lsr,
        2 => ShiftKind::Asr,
        _ => return Err(DecodeError::InvalidShift),
    };

    let width = if sf == 1 {
        BitWidth::W64
    } else {
        BitWidth::W32
    };

    let max_shift = if width == BitWidth::W64 { 63u32 } else { 31u32 };
    if imm6 > max_shift {
        return Err(DecodeError::InvalidShift);
    }

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
        rd: to_reg(rd as u8, width)?,
        rn: to_reg(rn as u8, width)?,
        rm: to_reg(rm as u8, width)?,
        shift: shift_kind,
        shift_amount: imm6,
        imm: 0,
        n: 0,
        width,
    })
}
