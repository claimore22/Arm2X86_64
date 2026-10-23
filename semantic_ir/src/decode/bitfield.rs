use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_bitfield(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let opc = (bits >> 29) & 3;
    let n = (bits >> 22) & 1;
    let immr = (bits >> 16) & 0x1F;
    let imms = (bits >> 10) & 0x3F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    if opc == 3 {
        return Err(DecodeError::UndefinedInstruction);
    }
    let mnemonic = match opc {
        0 => "SBFM",
        1 => "BFM",
        2 => "UBFM",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    let width = if sf == 1 {
        BitWidth::W64
    } else {
        BitWidth::W32
    };

    if sf == 0 && n != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }
    // For W32, immr/imms must be <32
    if width == BitWidth::W32 {
        if immr > 31 || imms > 31 {
            return Err(DecodeError::InvalidShift);
        }
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
        rm: Reg::Xzr, // unused
        shift: ShiftKind::Lsl,
        shift_amount: immr,
        imm: imms,
        n: n as u8,
        width,
    })
}
