mod common;
use semantic_ir::codegen::{lower_to_x86_64, lower_to_x86_64_with_flags};
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};
use common::extract_arm_flags_from_x86;

#[test]
fn adc_sbc_flags() {
    // ADC X0, X1, X2 = 0x90820020 (sf=1, op=0 for ADC, bit28=1, bit23=1, rm=2, rn=1, rd=0)
    let bits = 0x90820020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ADC");

    
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    // Test ADC without carry-in: 10 + 20 = 30
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp.state.pstate.flags.c = false; // no carry in
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 30);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(!interp.state.pstate.flags.c); // no carry out
    assert!(!interp.state.pstate.flags.v);
    
    // Native with flags - carry_in = 0
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 10, 20).unwrap();
    assert_eq!(native_result, 30);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    // ADC 10 + 20 = 30: N=0, Z=0, C=0, V=0
    assert_eq!(native_arm_flags, 0);
    
    // Test SBC without carry-in: 30 - 10 = 20
    // SBC X0, X1, X2 = 0xD0820020 (sf=1, op=1 for SBC, bit28=1, bit23=1, rm=2, rn=1, rd=0)
    let bits = 0xD0820020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SBC");

    
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 30);
    interp.state.gpr.write_x(2, 10);
    interp.state.pstate.flags.c = true; // no borrow in (carry=1 means no borrow for sub)
    interp.execute_instruction(&instruction).unwrap();
    // SBC: 30 - 10 = 20 (with carry=1 meaning no borrow)
    assert_eq!(interp.state.gpr.read_x(0), 20);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(interp.state.pstate.flags.c); // no borrow
    assert!(!interp.state.pstate.flags.v);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 30, 10).unwrap();
    assert_eq!(native_result, 20);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, true);
    assert_eq!(native_arm_flags & 0b0010, 0b0010); // C=1
}

#[test]
fn adc_sbc_edge_cases() {
    // ADC with carry out (no carry in)
    let bits = 0x90820020u32; // ADC X0, X1, X2
    let decoded = decode_instruction(bits).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xFFFFFFFFFFFFFFFFu64);
    interp.state.gpr.write_x(2, 1);
    interp.state.pstate.flags.c = false; // no carry in
    interp.execute_instruction(&instruction).unwrap();
    // 0xFFFFFFFFFFFFFFFF + 1 = 0 (with carry out)
    assert_eq!(interp.state.gpr.read_x(0), 0);
    assert!(!interp.state.pstate.flags.n);
    assert!(interp.state.pstate.flags.z);
    assert!(interp.state.pstate.flags.c); // carry out
    assert!(!interp.state.pstate.flags.v);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 0xFFFFFFFFFFFFFFFFu64, 1).unwrap();
    assert_eq!(native_result, 0);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags & 0b0010, 0b0010); // C=1
    assert_eq!(native_arm_flags & 0b0100, 0b0100); // Z=1
    
    // SBC with borrow (no carry in = borrow in)
    let bits = 0xD0820020u32; // SBC X0, X1, X2
    let decoded = decode_instruction(bits).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0);
    interp.state.gpr.write_x(2, 1);
    interp.state.pstate.flags.c = false; // carry=0 means borrow in
    interp.execute_instruction(&instruction).unwrap();
    // SBC: 0 - 1 - 1 = 0xFFFFFFFFFFFFFE (with borrow)
    assert_eq!(interp.state.gpr.read_x(0), 0xFFFFFFFFFFFFFEu64);
    assert!(interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(!interp.state.pstate.flags.c); // borrow occurred
    assert!(!interp.state.pstate.flags.v);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 0, 1).unwrap();
    assert_eq!(native_result, 0xFFFFFFFFFFFFFEu64);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, true);
    assert_eq!(native_arm_flags & 0b0010, 0); // C=0 (borrow)
    assert_eq!(native_arm_flags & 0b1000, 0b1000); // N=1
}

#[test]
fn adc_sbc_32bit() {
    // ADC W0, W1, W2 = 0x10820020 (sf=0, op=0 for ADC, bit28=1, bit23=1, rm=2, rn=1, rd=0)
    let bits = 0x10820020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ADC");
    assert_eq!(decoded.width, BitWidth::W32);
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_w(1, 10);
    interp.state.gpr.write_w(2, 20);
    interp.state.pstate.flags.c = false;
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_w(0), 30);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 10, 20).unwrap();
    assert_eq!(native_result, 30);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags, 0);
    
    // SBC W0, W1, W2 = 0x50820020 (sf=0, op=1 for SBC, bit28=1, bit23=1, rm=2, rn=1, rd=0)
    let bits = 0x50820020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SBC");
    assert_eq!(decoded.width, BitWidth::W32);
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_w(1, 30);
    interp.state.gpr.write_w(2, 10);
    interp.state.pstate.flags.c = true; // no borrow
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_w(0), 20);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 30, 10).unwrap();
    assert_eq!(native_result, 20);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, true);
    assert_eq!(native_arm_flags & 0b0010, 0b0010); // C=1
}
