mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::Reg;

#[test]
fn test_rev() {
    // REV X0, X1 = 0xDAE00020 (sf=1, opc=10, N=1, op2=00000, Rn=1, Rd=0)
    let bits = 0xDAE00020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "REV");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
}

#[test]
fn test_rev16() {
    // REV16 X0, X1 = op2=00001
    let bits = 0xDAE04020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "REV16");
}

#[test]
fn test_rev32() {
    // REV32 X0, X1 = op2=00010
    let bits = 0xDAE08020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "REV32");
}

#[test]
fn test_rev64() {
    // REV64 X0, X1 = op2=00011
    let bits = 0xDAE0C020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "REV64");
}

#[test]
fn test_clz() {
    // CLZ X0, X1 = op2=00100
    let bits = 0xDAE10020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "CLZ");
}

#[test]
fn test_cls() {
    // CLS X0, X1 = op2=00101
    let bits = 0xDAE14020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "CLS");
}

#[test]
fn test_rbit() {
    // RBIT X0, X1 = op2=00110
    let bits = 0xDAE18020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "RBIT");
}