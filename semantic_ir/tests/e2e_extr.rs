mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::{Reg, BitWidth};

#[test]
fn test_extr() {
    // EXTR X0, X1, X2, #8
    let bits = 0xE3C22020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "EXTR");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.shift_amount, 8);
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn test_extr_32bit() {
    // EXTR W0, W1, W2, #4
    // SF=0, opc=11, N=0, op2=001111, Rm=2, imms=4, Rn=1, Rd=0
    // bit 31: sf=0
    // bits 30-29: opc=11 (3)
    // bit 28: N=0
    // bits 27-22: op2=001111 (0xF)
    // bits 21-16: Rm=2
    // bits 15-10: imms=4
    // bits 9-5: Rn=1
    // bits 4-0: Rd=0
    // = 0x63C21020
    let bits = 0x63C21020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "EXTR");
    assert_eq!(decoded.rd, Reg::W(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.shift_amount, 4);
    assert_eq!(decoded.width, BitWidth::W32);
}