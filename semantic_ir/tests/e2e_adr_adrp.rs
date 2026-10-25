mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::{Reg, BitWidth};

#[test]
fn test_adr() {
    // ADR X0, label (+8) = 0x10000000
    // bits 31-24: 00010000 (0x10)
    // bits 23-5: immhi = 8 (19 bits, shifted by 2) = 0x00008
    // bit 4: 0 (immlo bit 1)
    // bits 3-0: Rd = 0
    // = 0x10000000
    let bits = 0x10000000u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ADR");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_adrp() {
    // ADRP X0, label (+4096) 
    // bits 31-24: 10010000 (0x90)
    // bits 23-5: immhi = 1 (19 bits, shifted by 12) = 0x00001
    // bit 4: 0 (immlo bit 1)
    // bits 3-0: Rd = 0
    // = 0x90000000
    let bits = 0x90000000u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ADRP");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.width, BitWidth::W64);
}