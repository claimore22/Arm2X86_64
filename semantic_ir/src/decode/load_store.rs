use crate::types::{BitWidth, MemAccess, MemOrdering, MemSize, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_ldr_str(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    // LDR/STR (unsigned offset): top 6 bits = 111001 for 64-bit (size=11)
    // Format: size(2) 111 0 01 0 . imm12(12) . Rn(5) . Rt(5)
    // opc (bits 23-22) determines load/store: 01=LDR, 00=STR
    
    let size = (bits >> 30) & 3;
    let opc = (bits >> 22) & 3;
    let imm12 = (bits >> 10) & 0xFFF;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    
    // Check fixed pattern: bits 29-24 = 111001
    if ((bits >> 24) & 0x3F) != 0b111001 {
        return Err(DecodeError::UndefinedInstruction);
    }
    
    // opc: 00=STR, 01=LDR
    if opc != 0b00 && opc != 0b01 {
        return Err(DecodeError::UndefinedInstruction);
    }
    
    let mnemonic = if opc == 0b00 { "STR" } else { "LDR" };
    
    let width = match size {
        0 => BitWidth::W8,
        1 => BitWidth::W16,
        2 => BitWidth::W32,
        _ => BitWidth::W64,
    };
    
    // Register to load/store from Rt
    let rd = match Reg::x(rt as u8) {
        Some(Reg::X(n)) => {
            if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
                if n == 31 { Reg::Wzr } else { Reg::W(n) }
            } else {
                if n == 31 { Reg::Xzr } else { Reg::X(n) }
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
    
    // Base register Rn
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => {
            if n == 31 { Reg::Xzr } else { Reg::X(n) }
        }
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };
    
    let byte_offset = imm12 << size;
    
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd,
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: byte_offset,
        n: 0,
        width,
    })
}