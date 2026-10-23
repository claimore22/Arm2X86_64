use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;
use crate::primitives::ShiftKind;

pub fn decode_logical_imm(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let opc = (bits >> 29) & 3;
    let n = (bits >> 22) & 1;
    let immr = (bits >> 16) & 0x3F;
    let imms = (bits >> 10) & 0x3F;
    let rn = (bits >> 5) & 0x1F;
    let rd = bits & 0x1F;

    if sf == 0 {
        return Err(DecodeError::UndefinedInstruction); // W32 not yet supported (decode ambiguity)
    }
    if n == 0 && (immr & 0x20) == 0 {
        // require bit21=1 for 64-bit to avoid collision with ADD shifted
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = match opc {
        0 => "ANDI",
        1 => "ORRI",
        2 => "EORI",
        3 => "ANDSI",
        _ => return Err(DecodeError::UndefinedInstruction),
    };

    let width = BitWidth::W64;

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
        rm: Reg::Xzr,
        shift: ShiftKind::Lsl,
        shift_amount: immr,
        imm: imms,
        n: n as u8,
        width,
    })
}
