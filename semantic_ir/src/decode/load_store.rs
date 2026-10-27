use crate::types::{BitWidth, Reg};
use crate::primitives::ShiftKind;
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_load_store(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    // LDR/STR (unsigned offset): top 6 bits = 111001
    if ((bits >> 24) & 0x3F) != 0b111001 {
        return Err(DecodeError::UndefinedInstruction);
    }
    let size_bits = (bits >> 30) & 3;
    let opc = (bits >> 22) & 3; // 00=STR, 01=LDR, 10=?? (STR variants), 11=LDR PRFM
    if opc != 0b00 && opc != 0b01 {
        return Err(DecodeError::UndefinedInstruction);
    }
    let imm12 = (bits >> 10) & 0xFFF;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;

    let width = match size_bits {
        0 => BitWidth::W8,
        1 => BitWidth::W16,
        2 => BitWidth::W32,
        _ => BitWidth::W64,
    };
    let scale = size_bits;
    let byte_offset = imm12 << scale;

    let mnemonic = if opc == 0b00 { "STR" } else { "LDR" };

    let rd = match Reg::x(rt as u8) {
        Some(r) if r != Reg::Xzr && r != Reg::X(0) => {
            // keep width variant
            if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
                match r {
                    Reg::X(n) => Reg::W(n),
                    Reg::Xzr => Reg::Wzr,
                    other => other,
                }
            } else {
                r
            }
        }
        Some(Reg::X(n)) => {
            if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
                if n == 0 { Reg::Wzr } else { Reg::W(n) }
            } else {
                if n == 0 { Reg::Xzr } else { Reg::X(n) }
            }
        }
        Some(Reg::Xzr) => {
            if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
                Reg::Wzr
            } else {
                Reg::Xzr
            }
        }
        _ => return Err(DecodeError::InvalidRegister),
    };

    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => {
            if n == 31 { Reg::Xzr } else { Reg::X(n) }
        }
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd,
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: ShiftKind::Lsl,
        shift_amount: 0,
        imm: byte_offset,
        n: 0,
        width,
    })
}