use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_branch_imm(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let _sf = (bits >> 31) & 1;
    let _op = (bits >> 31) & 1;  // 0 = B, 1 = BL? No, B and BL have different top bits
    
    // B: bits 31-26 = 000101 (0x05)
    // BL: bits 31-26 = 100101 (0x25)
    // B.cond: bits 31-24 = 01010100 (0x54), bits 3-0 = condition
    
    let top6 = (bits >> 26) & 0x3F;
    let top8 = (bits >> 24) & 0xFF;
    
    let mnemonic = if top6 == 0b000101 {
        "B"
    } else if top6 == 0b100101 {
        "BL"
    } else if top8 == 0x54 {
        "B.cond"
    } else {
        return Err(DecodeError::UndefinedInstruction);
    };

    // Verify fixed bits
    if top6 == 0b000101 || top6 == 0b100101 {
        // B/BL: bits 30-26 = 00101, bit 31 = 0 (B) or 1 (BL)
        // Already verified by top6 check
    } else {
        // B.cond: bits 31-24 = 01010100, bits 3-0 = condition
        if ((bits >> 4) & 1) != 0 {
            return Err(DecodeError::UndefinedInstruction);
        }
    }

    let width = BitWidth::W64;

    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: Reg::Xzr,  // Not used for branch
        rn: Reg::Xzr,  // Not used
        rm: Reg::Xzr,  // Not used
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: (bits >> 5) & 0x7FFFFFF,  // Store offset as immediate
        n: 0,
        width,
    })
}