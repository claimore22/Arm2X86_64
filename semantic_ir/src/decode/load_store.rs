use crate::types::{BitWidth, MemAccess, MemOrdering, MemSize, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_load_store(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    // LDR/STR (unsigned offset): opc=01 (LDR), 00 (STR), 10 (LDR with ordering?), 11 (STR with ordering?)
    // size: bits 31-30 (00=byte, 01=half, 10=word, 11=double)
    // bits 29-22: 111001 (fixed for unsigned offset)
    
    let size = (bits >> 30) & 3;
    let opc = (bits >> 22) & 3;
    let imm12 = (bits >> 10) & 0xFFF;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    
    // Fixed pattern: bits 29-24 = 111001
    if ((bits >> 24) & 0x3F) != 0b111001 {
        return Err(DecodeError::UndefinedInstruction);
    }
    
    let mnemonic = match opc {
        0b00 => "STR",
        0b01 => "LDR",
        0b10 => "STR",  // reserved in ARM ARM but common
        0b11 => "LDR",  // reserved in ARM ARM but common
        _ => return Err(DecodeError::UndefinedInstruction),
    };
    
    let width = match size {
        0 => BitWidth::W8,
        1 => BitWidth::W16,
        2 => BitWidth::W32,
        _ => BitWidth::W64,
    };
    
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 => {
            if n == 31 { Reg::Wzr } else { Reg::W(n) }
        }
        Some(Reg::X(n)) => Reg::X(n),
        Some(Reg::Xzr) => {
            if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
                Reg::Wzr
            } else {
                Reg::Xzr
            }
        }
        Some(_) => Reg::Xzr,
        None => return Err(DecodeError::InvalidRegister),
    };
    
    let rd = match Reg::x(rt as u8) {
        Some(Reg::X(n)) if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 => {
            if n == 31 { Reg::Wzr } else { Reg::W(n) }
        }
        Some(Reg::X(n)) => Reg::X(n),
        Some(Reg::Xzr) => {
            if width == BitWidth::W8 || width == BitWidth::W16 || width == BitWidth::W32 {
                Reg::Wzr
            } else {
                Reg::Xzr
            }
        }
        Some(_) => Reg::Xzr,
        None => return Err(DecodeError::InvalidRegister),
    };
    
    // Byte offset = imm12 * (1 << size)
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