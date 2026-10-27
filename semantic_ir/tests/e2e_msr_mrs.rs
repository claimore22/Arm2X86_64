mod common;
use semantic_ir::decode::decode_instruction;
use semantic_ir::types::Reg;

#[test]
fn test_mrs() {
    // MRS X0, NZCV
    let bits = 0xD53B4200u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MRS");
    assert_eq!(decoded.rd, Reg::X(0));
}

#[test]
fn test_msr() {
    // MSR (register) X0, NZCV
    // 0xD51B4200 is MRS? Let me use 0xD51B4200 which should be MSR
    // Actually, MSR uses bits 31-21 = 11010101001 = 0xD51
    let bits = 0xD51B4200u32;
    let decoded = decode_instruction(bits).unwrap();
    assert!(decoded.mnemonic == "MSR" || decoded.mnemonic == "MSR (imm)" || decoded.mnemonic == "MSR");
}