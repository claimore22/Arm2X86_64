use crate::types::{BitWidth, Reg};
use super::DecodeError;
use super::DecodedInstruction;

pub fn decode_ldxr(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    if ((bits >> 24) & 0x3F) != 0b001000 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 23) & 1) != 0 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 22) & 1) != 1 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 21) & 1) != 0 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 16) & 0x1F) != 0b11111 { return Err(DecodeError::UndefinedInstruction); }
    let size = (bits >> 30) & 3;
    if size != 2 && size != 3 { return Err(DecodeError::UndefinedInstruction); }
    let width = if size == 2 { BitWidth::W32 } else { BitWidth::W64 };
    let o0 = ((bits >> 15) & 1) == 1;
    // bits14-10 must be all ones
    if ((bits >> 10) & 0x1F) != 0b11111 { return Err(DecodeError::UndefinedInstruction); }
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    let mnemonic = if o0 { "LDAXR" } else { "LDXR" };
    let rd_reg = match Reg::x(rt as u8) {
        Some(Reg::X(n)) if width==BitWidth::W32 => Reg::W(n),
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => if n==31 { Reg::Xzr } else { Reg::X(n) },
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: rd_reg,
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: 0,
        n: 0,
        width,
    })
}

pub fn decode_stxr(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    if ((bits >> 24) & 0x3F) != 0b001000 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 23) & 1) != 0 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 22) & 1) != 0 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 21) & 1) != 0 { return Err(DecodeError::UndefinedInstruction); }
    let size = (bits >> 30) & 3;
    if size != 2 && size != 3 { return Err(DecodeError::UndefinedInstruction); }
    let width = if size == 2 { BitWidth::W32 } else { BitWidth::W64 };
    let o0 = ((bits >> 15) & 1) == 1;
    if ((bits >> 10) & 0x1F) != 0b11111 { return Err(DecodeError::UndefinedInstruction); }
    let rs = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    let mnemonic = if o0 { "STLXR" } else { "STXR" };
    let rd_reg = match Reg::x(rt as u8) {
        Some(Reg::X(n)) if width==BitWidth::W32 => Reg::W(n),
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => if n==31 { Reg::Xzr } else { Reg::X(n) },
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };
    // store status register number in imm
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: rd_reg,
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: rs as u32,
        n: rs as u8,
        width,
    })
}
pub fn decode_swp(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    if ((bits >> 24) & 0x3F) != 0b111000 { return Err(DecodeError::UndefinedInstruction); }
    let size = (bits >> 30) & 3;
    if size != 2 && size != 3 { return Err(DecodeError::UndefinedInstruction); }
    let width = if size == 2 { BitWidth::W32 } else { BitWidth::W64 };
    // bits15-10 = 100000
    if ((bits >> 10) & 0x3F) != 0b100000 { return Err(DecodeError::UndefinedInstruction); }
    // bit21 = 1
    if ((bits >> 21) & 1) != 1 { return Err(DecodeError::UndefinedInstruction); }
    let a = ((bits >> 23) & 1) == 1;
    let r = ((bits >> 22) & 1) == 1;
    let rs = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    let mnemonic = match (a, r) {
        (false, false) => "SWP",
        (true, false) => "SWPA",
        (false, true) => "SWPL",
        (true, true) => "SWPAL",
    };
    let rd_reg = match Reg::x(rt as u8) {
        Some(Reg::X(n)) if width==BitWidth::W32 => Reg::W(n),
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => if n==31 { Reg::Xzr } else { Reg::X(n) },
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };
    let d = DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: rd_reg,
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: rs as u32,
        n: rs as u8,
        width,
    };
    let _ = (a, r);
    Ok(d)
}

pub fn decode_cas(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    if ((bits >> 24) & 0x3F) != 0b001000 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 23) & 1) != 1 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 21) & 1) != 1 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 10) & 0x1F) != 0b11111 { return Err(DecodeError::UndefinedInstruction); }
    let size = (bits >> 30) & 3;
    if size != 2 && size != 3 { return Err(DecodeError::UndefinedInstruction); }
    let width = if size == 2 { BitWidth::W32 } else { BitWidth::W64 };
    let a = ((bits >> 22) & 1) == 1;
    let r = ((bits >> 15) & 1) == 1;
    let rs = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    let mnemonic = match (a, r) {
        (false, false) => "CAS",
        (true, false) => "CASA",
        (false, true) => "CASL",
        (true, true) => "CASAL",
    };
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => if n==31 { Reg::Xzr } else { Reg::X(n) },
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };
    // Compare register s is in Rs field and should be Rt? For CAS, Rs field holds compare. Store its index in imm.
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: match Reg::x(rt as u8) { // swap register (Rt) stored in rd
            Some(Reg::X(n)) if width==BitWidth::W32 => Reg::W(n),
            Some(r) => r,
            None => return Err(DecodeError::InvalidRegister),
        },
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: rs as u32, // compare register index
        n: rs as u8,
        width,
    })
}

pub fn decode_casp(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    // CASP always has bit31 = 0
    if ((bits >> 31) & 1) != 0 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 24) & 0x3F) != 0b001000 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 23) & 1) != 1 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 21) & 1) != 1 { return Err(DecodeError::UndefinedInstruction); }
    if ((bits >> 10) & 0x1F) != 0b11111 { return Err(DecodeError::UndefinedInstruction); }
    let sz = ((bits >> 30) & 1) as usize;
    let width = if sz == 0 { BitWidth::W32 } else { BitWidth::W64 };
    let a = ((bits >> 22) & 1) == 1;
    let r = ((bits >> 15) & 1) == 1;
    let rs = (bits >> 16) & 0x1F;
    let rn = (bits >> 5) & 0x1F;
    let rt = bits & 0x1F;
    // even register constraints
    if rs % 2 != 0 || rt % 2 != 0 { return Err(DecodeError::InvalidRegister); }
    if rs == 31 || rt == 31 { return Err(DecodeError::InvalidRegister); }
    let mnemonic = match (a, r) {
        (false, false) => "CASP",
        (true, false) => "CASPA",
        (false, true) => "CASPL",
        (true, true) => "CASPAL",
    };
    let rn_reg = match Reg::x(rn as u8) {
        Some(Reg::X(n)) => if n==31 { Reg::Xzr } else { Reg::X(n) },
        Some(Reg::Xzr) => Reg::Xzr,
        _ => return Err(DecodeError::InvalidRegister),
    };
    let rd_reg = match Reg::x(rt as u8) {
        Some(r) => r,
        None => return Err(DecodeError::InvalidRegister),
    };
    // stash compare register index in imm
    Ok(DecodedInstruction {
        mnemonic: mnemonic.to_string(),
        rd: rd_reg,
        rn: rn_reg,
        rm: Reg::Xzr,
        shift: crate::primitives::ShiftKind::Lsl,
        shift_amount: 0,
        imm: rs as u32,
        n: rs as u8,
        width,
    })
}
