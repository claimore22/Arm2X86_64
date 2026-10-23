use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_addsub_imm(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 30) & 1; // 0 add, 1 sub
    let s = (bits >> 29) & 1; // 1 => flags updated
    let sh = (bits >> 22) & 1;
    let imm12 = (bits >> 10) & 0xFFF;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    // Distinct mnemonics keep immediate and shifted-register forms apart.
    let mnemonic = match (op, s) {
        (0, 0) => "ADDI",
        (1, 0) => "SUBI",
        (0, 1) => "ADDSI",
        _ => "SUBSI",
    };

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    let imm = if sh == 1 { (imm12 as u32) << 12 } else { imm12 as u32 };

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
        rm: Reg::Xzr,
        shift: ShiftKind::Lsl,
        shift_amount: 0,
        imm,
        n: 0,
        width,
    })
}
