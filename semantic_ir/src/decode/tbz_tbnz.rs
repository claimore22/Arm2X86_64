use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_tbz_tbnz(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let sf = (bits >> 31) & 1;
    let op = (bits >> 24) & 1;  // 0 = TBZ, 1 = TBNZ
    let b5 = (bits >> 22) & 1;  // bit 5 of bit position
    let b40 = (bits >> 19) & 0x1F;  // bits 4-0 of bit position
    let imm14 = (bits >> 5) & 0x3FFF;  // 14-bit signed offset
    let rt = bits & 0x1F;

    // Verify fixed bits: bits 30-24 = 011011x (0x6E for TBZ, 0x6F for TBNZ)
    let top7 = (bits >> 24) & 0x7F;
    if top7 != 0x6E && top7 != 0x6F {
        return Err(DecodeError::UndefinedInstruction);
    }

    // Verify bit 23 = 0 (reserved)
    if ((bits >> 23) & 1) != 0 {
        return Err(DecodeError::UndefinedInstruction);
    }

    let mnemonic = if op == 0 { "TBZ" } else { "TBNZ" };
    let width = if sf == 1 { BitWidth::W64 } else { BitWidth::W32 };

    // Validate bit position
    let bit_pos = (b5 << 5) | b40;
    if width == BitWidth::W32 && bit_pos >= 32 {
        return Err(DecodeError::InvalidShift);
    }
    if width == BitWidth::W64 && bit_pos >= 64 {
        return Err(DecodeError::InvalidShift);
    }

    // Sign-extend the 14-bit offset
    let offset = if (imm14 & 0x2000) != 0 {
        (imm14 | 0xC000) as i32
    } else {
        imm14 as i32
    };

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
        rd: to_reg(rt as u8, width)?,
        rn: Reg::Xzr,  // Not used for TBZ/TBNZ
        rm: Reg::Xzr,  // Not used
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: bit_pos,
        imm: (offset as u32) & 0xFFFFFFFF,  // Store offset as immediate
        n: 0,
        width,
    })
}