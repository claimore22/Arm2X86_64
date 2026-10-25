use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_extr(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let opc = (bits >> 29) & 3;
    let n = (bits >> 28) & 1;
    let op2 = (bits >> 22) & 0x3F;
    let rm = (bits >> 16) & 0x1F;
    let imms = (bits >> 10) & 0x3F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    // EXTR: SF opc N 0 0 1 1 1 1 1 (bits 31-22 = SF 11 0 001111 = 0bSF 11 0 001111)
    // Verify fixed bits: opc=11, N=0, op2=0b001111=0xF
    if opc != 0b11 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if n != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }
    if op2 != 0b001111 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = "EXTR";

    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    // imms must be < width
    if imms >= width.bits() as u32 {
        return Err(DecodeError::InvalidShift);
    }

    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
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
        rd: to_reg(rd as u8, width)?,
        rn: to_reg(rn as u8, width)?,
        rm: to_reg(rm as u8, width)?,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: imms,
        imm: 0,
        n: 0,
        width,
    })
}