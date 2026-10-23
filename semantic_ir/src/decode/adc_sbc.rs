use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_adc_sbc(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 30) & 1; // 0 adc, 1 sbc
    let rm = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;
    let bit10 = (bits >> 10) & 1;
    if bit10 != 0 {
        return Err(DecodeError::UndefinedInstruction); // ADCS/SBCS not supported
    }
    if ((bits >> 28) & 1) != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 23) & 1) != 1 {
        return Err(DecodeError::UndefinedInstruction);
    }
    let mnemonic = if op == 0 { "ADC" } else { "SBC" };
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
