use crate::types::{BitWidth, Reg};
use crate::primitives::ShiftKind;
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_load_store_reg(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    if ((bits >> 24) & 0x3F) != 0x38 {
        return Err(DecodeError::UndefinedInstruction);
    }
    let size_bits = (bits >> 30) & 3;
    let load = (bits >> 22) & 1 == 1;
    let rm = (bits >> 16) & 0x1F;
    let option = (bits >> 13) & 7;
    let s = (bits >> 12) & 1;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;

    let width = match size_bits {
        0 => BitWidth::W8,
        1 => BitWidth::W16,
        2 => BitWidth::W32,
        _ => BitWidth::W64,
    };
    let mnemonic = if load { "LDR" } else { "STR" };

    let rd = if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
        let r = match Reg::x(rt as u8) { Some(Reg::X(n)) => Reg::W(n), Some(Reg::Xzr) => Reg::Wzr, _ => return Err(DecodeError::InvalidRegister) };
        r
    } else {
        match Reg::x(rt as u8) { Some(Reg::X(n)) if n != 0 => Reg::X(n), Some(Reg::Xzr) => Reg::Xzr, Some(Reg::X(0)) => Reg::Xzr, _ => return Err(DecodeError::InvalidRegister) }
    };
    let rn_reg = match Reg::x(rn as u8) { Some(Reg::X(n)) if n != 31 => Reg::X(n), _ => Reg::Xzr };
    let rm_reg = match Reg::x(rm as u8) { Some(Reg::X(n)) if n != 0 => Reg::X(n), _ => Reg::Xzr };

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd,
        rn: rn_reg,
        rm: rm_reg,
        shift: ShiftKind::Lsl,
        shift_amount: if s == 1 { size_bits } else { 0 },
        imm: option as u32,
        n: 0,
        width,
    })
}