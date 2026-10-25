use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_ccmp(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 30) & 1; // 0 ccmp, 1 ccmn
    let rm = (bits >> 16) & 0x1F;
    let nzcv = bits & 0xF;
    let cond = (bits >> 12) & 0xF;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F; // rn (same)
    if ((bits >> 24) & 0x1F) != 0x1A { return Err(DecodeError::UndefinedInstruction); }
    // bit23 not required for CCMP

    let mnemonic = if op == 0 { "CCMP" } else { "CCMN" };
    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };
    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
        match Reg::x(idx) {
            Some(Reg::X(n)) if w == BitWidth::W32 => Ok(Reg::W(n)),
            Some(Reg::Xzr) => { if w == BitWidth::W32 { Ok(Reg::Wzr) } else { Ok(Reg::Xzr) } }
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
        shift_amount: cond as u32, // store cond
        imm: nzcv, // store nzcv
        n: 0,
        width,
    })
}
