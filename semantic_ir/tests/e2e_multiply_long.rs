mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::{Reg, BitWidth};

#[test]
fn test_smull() {
    let bits = 0x9B627C20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SMULL");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_umull() {
    let bits = 0x9B227C20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "UMULL");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_smulh() {
    let bits = 0x9B427C20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SMULH");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_umulh() {
    let bits = 0x9BC27C20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "UMULH");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_smaddl() {
    let bits = 0x9B62FC20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SMADDL");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_umaddl() {
    let bits = 0x9B22FC20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "UMADDL");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.width, BitWidth::W64);
}