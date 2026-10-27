use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_msr_mrs(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let top11 = (bits >> 21) & 0x7FF;
    let rt = bits & 0x1F;

    // MRS: bits 31-21 = 11010101001 (0x6A9)
    // MSR (reg): bits 31-21 = 11010101000 (0x6A8)
    let mnemonic = if top11 == 0b11010101001 {
        "MRS"
    } else {
        "MSR"
    };

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: match Reg::x(rt as u8) {
            Some(r) => r,
            None => return Err(DecodeError::InvalidRegister),
        },
        rn: Reg::Xzr,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: 0,
        n: 0,
        width: BitWidth::W64,
    })
}