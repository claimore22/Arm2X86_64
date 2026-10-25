mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::Reg;

#[test]
fn test_neg() {
    // NEG X0, X1 = SUB X0, XZR, X1 (Rn=31, Rm=1)
    let bits = 0xD20103E0u32;  // SUB X0, XZR, X1, LSL #0
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SUB");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::Xzr);
    assert_eq!(decoded.rm, Reg::X(1));
}

#[test]
fn test_negs() {
    // NEGS X0, X1 = SUBS X0, XZR, X1 (Rd=XZR, Rd=31)
    let bits = 0xEB0103E0u32;  // SUBS X0, XZR, X1, LSL #0
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SUBS");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::Xzr);
    assert_eq!(decoded.rm, Reg::X(1));
}

#[test]
fn test_mvn() {
    // MVN X0, X1 = ORN X0, XZR, X1
    // ORN: sf=1, opc=01, N=0, bits 28-24=10100 (0x14), bit 23=0, Rm=1, Rn=31, Rd=0
    // = 0x80000000 | 0x20000000 | 0x14000000 | 0x10000 | 0x3E0 | 0
    // = 0xB40103E0
    let bits = 0xB42103E0u32;  // ORN X0, XZR, X1 (bit 21 = 1 for NOT)
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ORN");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::Xzr);
    assert_eq!(decoded.rm, Reg::X(1));
}

#[test]
fn test_ngc() {
    // NGC X0, X1 = SBC X0, XZR, X1
    // SBC: sf=1, op=1, S=0, bits 28-24=11010, bit23=0, Rm=1, Rn=31 (XZR), Rd=0
    // = 0x80000000 | 0x40000000 | 0x1A000000 | 0x10000 | 0x3E0 | 0
    // = 0xDA0103E0
    let bits = 0xDA0103E0u32;  // SBC X0, XZR, X1
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SBC");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::Xzr);
    assert_eq!(decoded.rm, Reg::X(1));
}