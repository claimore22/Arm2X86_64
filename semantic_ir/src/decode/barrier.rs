use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_barrier(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    // Barrier instructions: ISB, DSB, DMB
    // bits 31-24 = 0xD5 (system instruction)
    // bits 23-20 = 0 (reserved)
    // bits 19-16 = 3 (op1)
    // bits 15-12 = 3 (CRn)
    // bits 11-8 = CRm (scope: F=SY, E=ST, etc.)
    // bits 7-5 = op2 (4=DSB, 5=DMB, 6=ISB)
    // bits 4-0 = 1F (Rt, system instruction form)
    
    let op2 = (bits >> 5) & 7;
    let crm = (bits >> 8) & 0xF;
    let rt = bits & 0x1F;
    
    // Verify fixed bits
    if (bits >> 24) != 0xD5 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 20) & 0xF) != 0 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 16) & 7) != 3 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 12) & 0xF) != 3 { return Err(DecodeError::UndefinedInstruction); }
    if rt != 0x1F { return Err(DecodeError::UndefinedInstruction); }
    
    let mnemonic = match op2 {
        4 => "DSB",
        5 => "DMB",
        6 => "ISB",
        _ => return Err(DecodeError::UndefinedInstruction),
    };
    
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: Reg::Xzr,
        rn: Reg::Xzr,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: crm as u32,  // Store CRm (scope, e.g. SY=0xF) as immediate
        n: 0,
        width: BitWidth::W64,
    })
}