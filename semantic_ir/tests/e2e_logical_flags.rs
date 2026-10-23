mod common;
use semantic_ir::codegen::{lower_to_x86_64, lower_to_x86_64_with_flags};
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};
use common::extract_arm_flags_from_x86;

#[test]
fn ands_orr_eors_bics_flags() {
    // ANDS X0, X1, X2 = 0x8A020020
    let bits = 0x8A020020u32;
    let decoded = decode_instruction(bits).unwrap();

    
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    // Test ANDS: 0xF0 & 0x0F = 0x00
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xF0);
    interp.state.gpr.write_x(2, 0x0F);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 0x00);
    assert!(!interp.state.pstate.flags.n); // result is 0
    assert!(interp.state.pstate.flags.z);  // result is 0
    assert!(!interp.state.pstate.flags.c); // logical ops clear C
    assert!(!interp.state.pstate.flags.v); // logical ops clear V
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 0xF0, 0x0F).unwrap();
    assert_eq!(native_result, 0x00);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags & 0b0100, 0b0100); // Z=1
    assert_eq!(native_arm_flags & 0b1000, 0); // N=0
    assert_eq!(native_arm_flags & 0b0010, 0); // C=0
    assert_eq!(native_arm_flags & 0b0001, 0); // V=0
    
    // Test ANDS with non-zero result: 0xFF & 0x0F = 0x0F
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xFF);
    interp.state.gpr.write_x(2, 0x0F);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 0x0F);
    assert!(!interp.state.pstate.flags.n); // positive
    assert!(!interp.state.pstate.flags.z); // non-zero
    
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 0xFF, 0x0F).unwrap();
    assert_eq!(native_result, 0x0F);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags & 0b1000, 0); // N=0
    assert_eq!(native_arm_flags & 0b0100, 0); // Z=0
    
    // Test BICS: X0 = X1 & ~X2
    // BICS X0, X1, X2 = 0xAA020020
    let bits = 0xAA020020u32;
    let decoded = decode_instruction(bits).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xFF);
    interp.state.gpr.write_x(2, 0x0F);
    interp.execute_instruction(&instruction).unwrap();
    // BICS: 0xFF & ~0x0F = 0xF0
    assert_eq!(interp.state.gpr.read_x(0), 0xF0);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 0xFF, 0x0F).unwrap();
    assert_eq!(native_result, 0xF0);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags & 0b1000, 0); // N=0
    assert_eq!(native_arm_flags & 0b0100, 0); // Z=0
}

#[test]
fn logical_shifted_with_shift_amount() {
    // ANDS X0, X1, X2, LSL #4
    // X2 = 0xF, shifted by 4 = 0xF0
    // 0xFF & 0xF0 = 0xF0
    let bits = 0x8A020420u32; // LSL #4
    let decoded = decode_instruction(bits).unwrap();

    
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xFF);
    interp.state.gpr.write_x(2, 0xF);
    interp.execute_instruction(&instruction).unwrap();
    // 0xF << 4 = 0xF0, 0xFF & 0xF0 = 0xF0
    assert_eq!(interp.state.gpr.read_x(0), 0xF0);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 0xFF, 0xF).unwrap();
    assert_eq!(native_result, 0xF0);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags & 0b1000, 0); // N=0
    assert_eq!(native_arm_flags & 0b0100, 0); // Z=0
}

#[test]
fn adds_subs_32bit() {
    // ADDS W0, W1, W2, LSL #0
    let bits = 0x2B020020u32; // ADDS W0, W1, W2 (sf=0)
    let decoded = decode_instruction(bits).unwrap();

    
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_w(1, 10);
    interp.state.gpr.write_w(2, 20);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_w(0), 30);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(!interp.state.pstate.flags.c);
    assert!(!interp.state.pstate.flags.v);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 10, 20).unwrap();
    assert_eq!(native_result, 30);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags, 0);
    
    // SUBS W0, W1, W2
    let bits = 0x6B020020u32; // SUBS W0, W1, W2
    let decoded = decode_instruction(bits).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_w(1, 30);
    interp.state.gpr.write_w(2, 10);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_w(0), 20);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(interp.state.pstate.flags.c); // no borrow
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 30, 10).unwrap();
    assert_eq!(native_result, 20);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, true);
    assert_eq!(native_arm_flags & 0b0010, 0b0010); // C=1
}

#[test]
fn adc_sbc_32bit() {
    // ADC W0, W1, W2
    let bits = 0x0B000020u32; // ADC W0, W1, W2 (extend)
    let decoded = decode_instruction(bits).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    
    let mut interp = Interpreter::new();
    interp.state.gpr.write_w(1, 10);
    interp.state.gpr.write_w(2, 20);
    interp.state.pstate.flags.c = true;
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_w(0), 31);
    
    let bytes = lower_to_x86_64_with_flags(&decoded).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 10, 20).unwrap();
    assert_eq!(native_result, 31);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    assert_eq!(native_arm_flags, 0);
}
