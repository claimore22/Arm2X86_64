use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::{ExtendKind, ShiftKind};

pub fn decode_extend(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 30) & 1; // 0 add, 1 sub
    let option = (bits >> 21) & 7;
    let imm3 = (bits >> 10) & 7;
    let rm = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    if ((bits >> 24) & 0x1F) != 0x0B {
        return Err(DecodeError::UndefinedInstruction);
    }
    if ((bits >> 20) & 1) != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if imm3 > 4 {
        return Err(DecodeError::InvalidShift);
    }

    let _mnemonic = if op == 0 { "ADD (ext)" } else { "SUB (ext)" }; // distinct? Use ADDX/SUBX?
    // Use distinct mnemonics to avoid collision with shifted.
    let mnemonic = if op == 0 { "ADDX" } else { "SUBX" };

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    let _extend_kind = match option {
        0 => ExtendKind::Uxtb,
        1 => ExtendKind::Uxth,
        2 => ExtendKind::Uxtw,
        3 => ExtendKind::Uxtx,
        4 => ExtendKind::Sxtb,
        5 => ExtendKind::Sxth,
        6 => ExtendKind::Sxtw,
        7 => ExtendKind::Sxtx,
        _ => return Err(DecodeError::InvalidShift),
    };

    let rm_is_x = option == 3 || option == 7;
    let to_reg_rm = |idx: u8| -> Result<Reg, DecodeError> {
        if rm_is_x {
            match Reg::x(idx) {
                Some(Reg::X(n)) => Ok(Reg::X(n)),
                Some(Reg::Xzr) => Ok(Reg::Xzr),
                _ => Err(DecodeError::InvalidRegister),
            }
        } else {
            match Reg::x(idx) {
                Some(Reg::X(n)) => Ok(Reg::W(n)),
                Some(Reg::Xzr) => Ok(Reg::Wzr),
                _ => Err(DecodeError::InvalidRegister),
            }
        }
    };
    fn to_reg_w(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
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
        rd: to_reg_w(rd as u8, width)?,
        rn: to_reg_w(rn as u8, width)?,
        rm: to_reg_rm(rm as u8)?,
        shift: ShiftKind::Lsl,
        shift_amount: imm3,
        imm: option,
        n: 0,
        width,
    })
}
