use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_divide(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let _u = (bits >> 29) & 1; // 0 signed, 1 unsigned? Actually udiv has bit29=1? 
    let rm = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;
    // distinguish sdiv (0x9AC20C20) vs udiv (0x9AC20820): bit10? 
    let _mnemonic = if (bits & 0x00000400) != 0 { "SDIV" } else { "UDIV" };
    // Actually bit10=0 for both? Let's check: 0x9AC20C20 bit10 =? 0x0C20 bit10=0? 0x0C20=0000 1100 0010 0000, bit10=0. UDIV 0x9AC20820 bit10=0 too. Difference bit11? 0x0C vs 0x08: bit11? 0x0C=00001100, 0x08=00001000, bit11? bit2 of nibble = 1 for sdiv. So bit11=1 for sdiv.
    let mnemonic = if ((bits >> 10) & 1) == 1 { "SDIV" } else { "UDIV" };

    if ((bits >> 24) & 0x1F) != 0x1A {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 23) & 1) != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }

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
        shift_amount: 0,
        imm: 0,
        n: 0,
        width,
    })
}
