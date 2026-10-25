mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::{Reg, BitWidth};

#[test]
fn test_br() {
    // BR X1 = 0xD61F0020
    let bits = 0xD61F0020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "BR");
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_blr() {
    // BLR X1 = 0xD63F0020
    let bits = 0xD63F0020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "BLR");
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_ret() {
    // RET = 0xD65F03C0 (RET X30)
    let bits = 0xD65F03C0u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "RET");
    assert_eq!(decoded.rn, Reg::X(30));
    assert_eq!(decoded.width, BitWidth::W64);
}