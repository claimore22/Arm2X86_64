use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_move_wide(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let opc = (bits >> 29) & 3;
    let hw = (bits >> 21) & 3;
    let imm16 = (bits >> 5) & 0xFFFF;
    let rd = bits & 0x1F;

    let mnemonic = match opc {
        0 => "MOVN",
        2 => "MOVZ",
        3 => "MOVK",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    let width = if sf == 1 {
        BitWidth::W64
    } else {
        BitWidth::W32
    };

    let max_hw = if width == BitWidth::W64 { 3u32 } else { 1u32 };
    if hw > max_hw {
        return Err(DecodeError::UndefinedInstruction);
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
        rn: Reg::Xzr,
        rm: Reg::Xzr,
        shift: ShiftKind::Lsl,
        shift_amount: hw * 16,
        imm: imm16,
        n: 0,
        width,
    })
}
