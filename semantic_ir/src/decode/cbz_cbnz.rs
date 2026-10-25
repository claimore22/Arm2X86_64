use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_cbz_cbnz(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 24) & 1;  // 0 = CBZ, 1 = CBNZ
    let imm19 = (bits >> 5) & 0x7FFFF;  // 19-bit signed offset
    let rt = bits & 0x1F;

    // Verify fixed bits: bits 30-24 = 1011010 (0x5A for CBZ), 1011011 (0x5B for CBNZ)
    let top7 = (bits >> 24) & 0x7F;
    if top7 != 0x5A && top7 != 0x5B {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = if op == 0 { "CBZ" } else { "CBNZ" };
    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    // Sign-extend the 19-bit offset to 64 bits
    let offset = if (imm19 & 0x40000) != 0 {
        (imm19 | 0xFFF80000) as i32
    } else {
        imm19 as i32
    };

    fn to_reg(idx: u8, w: BitWidth) -> Result<Reg, DecodeError> {
        match crate::types::Reg::x(idx) {
            Some(crate::types::Reg::X(n)) if w == BitWidth::W32 => Ok(crate::types::Reg::W(n)),
            Some(crate::types::Reg::X(n)) => Ok(crate::types::Reg::X(n)),
            Some(crate::types::Reg::Xzr) => {
                if w == BitWidth::W32 {
                    Ok(crate::types::Reg::Wzr)
                } else {
                    Ok(crate::types::Reg::Xzr)
                }
            }
            Some(_) => Err(DecodeError::InvalidRegister),
            None => Err(DecodeError::InvalidRegister),
        }
    }

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: to_reg(rt as u8, width)?,
        rn: crate::types::Reg::Xzr,  // Not used for CBZ/CBNZ
        rm: crate::types::Reg::Xzr,  // Not used
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: (offset as u32) & 0xFFFFFFFF,  // Store offset as immediate
        n: 0,
        width,
    })
}