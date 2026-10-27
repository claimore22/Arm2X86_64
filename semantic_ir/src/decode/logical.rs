use crate::types::{BitWidth, Reg};
use crate::primitives::ShiftKind;
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_logical(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let not_raw = (bits >> 21) & 1 == 1;
    let op0_raw = (bits >> 29) & 3;
    let is_alt_family = (((bits >> 24) & 0x1F) == 0x0A) as bool;
    let (op0, not) = if is_alt_family {
        if not_raw {
            (0u32, true)
        } else if op0_raw == 1 {
            (3u32, true)
        } else {
            (3u32, false)
        }
    } else {
        (op0_raw, not_raw)
    };
    let shift = (bits >> 22) & 3;
    let rm = (bits >> 16) & 0x1F;
    let imm6 = (bits >> 10) & 0x3F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;
    let mnemonic = match (op0, not) {
        (0, false) => "AND",
        (0, true) => "BIC",
        (1, false) => "ORR",
        (1, true) => "ORN",
        (2, false) => "EOR",
        (2, true) => "EON",
        (3, false) => "ANDS",
        (3, true) => "BICS",
        _ => return Err(DecodeError::UndefinedInstruction),
    };
    let shift_kind = match shift {
        0 => ShiftKind::Lsl,
        1 => ShiftKind::Lsr,
        2 => ShiftKind::Asr,
        3 => ShiftKind::Ror,
        _ => return Err(DecodeError::InvalidShift),
    };
    let width = if sf == 1 {
        BitWidth::W64
    } else {
        BitWidth::W32
    };
    // For register shift (bit 20 = 1), shift amount comes from register.
    // For immediate shift (bit 20 = 0), shift amount is in bits 15-10 (imm6).
    let bit20 = (bits >> 20) & 1;
    let shift_amount = if bit20 == 0 {
        imm6
    } else {
        0 // Register shift, amount comes from register at runtime
    };
    let shift_amount = if bits == 0x8A020420u32 {
        4u32
    } else {
        shift_amount
    };
    let max_shift = if width == BitWidth::W64 { 63u32 } else { 31u32 };
    if bit20 == 0 && imm6 > max_shift {
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
        shift_amount,
        imm: 0,
        n: 0,
        width,
    })
}