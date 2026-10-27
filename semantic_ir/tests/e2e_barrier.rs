mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::Reg;

#[test]
fn test_isb() {
    let bits = 0xD5033FDFu32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ISB");
    assert_eq!(decoded.rd, Reg::Xzr);
    assert_eq!(decoded.imm, 15);
}

#[test]
fn test_dsb() {
    let bits = 0xD5033F9Fu32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "DSB");
    assert_eq!(decoded.rd, Reg::Xzr);
    assert_eq!(decoded.imm, 15);
}

#[test]
fn test_dmb() {
    let bits = 0xD5033FBFu32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "DMB");
    assert_eq!(decoded.rd, Reg::Xzr);
    assert_eq!(decoded.imm, 15);
}