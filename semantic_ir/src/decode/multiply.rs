use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_multiply(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 30) & 1; // 0 madd, 1 msub
    let rm = (bits >> 16) & 0x1F;
    let ra = (bits >> 10) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;
    let bit15 = (bits >> 15) & 1;
    if bit15 != 0 {
        return Err(DecodeError::UndefinedInstruction); // not madd/msub
    }

    let mnemonic = if ra == 31 {
        if op == 0 { "MUL" } else { "MNEG" }
    } else {
        if op == 0 { "MADD" } else { "MSUB" }
    };

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
        match Reg::x(idx) {
            Some(Reg::X(n)) if w == BitWidth::W32 => Ok(Reg::W(n)),
            Some(Reg::Xzr) => { if w == BitWidth::W32 { Ok(Reg::Wzr) } else { Ok(Reg::Xzr) } }
            Some(r) => Ok(r),
            None => Err(DecodeError::InvalidRegister),
        }
    }

    // We need store ra. DecodedInstruction has no ra field. Use rm? No, rm used. 
    // Store ra in imm field (0..31). rd/rn/rm real.
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: to_reg(rd as u8, width)?,
        rn: to_reg(rn as u8, width)?,
        rm: to_reg(rm as u8, width)?,
        shift: ShiftKind::Lsl,
        shift_amount: ra, // store ra here
        imm: 0,
        n: 0,
        width,
    })
}
