mod common;
use semantic_ir::codegen::{lower_to_x86_64, lower_to_x86_64_with_flags};
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};
use common::extract_arm_flags_from_x86;

#[test]
fn adds_subs() {
    // ADDS X0, X1, X2, LSL #0 = 0xAB020020
    let bits = 0xAB020020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ADDS");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 30);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(!interp.state.pstate.flags.c);
    assert!(!interp.state.pstate.flags.v);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 10, 20).unwrap();
    assert_eq!(native, 30);

    // Native with flags
    let bytes = lower_to_x86_64_with_flags(&decoded, false).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 10, 20, false).unwrap();
    assert_eq!(native_result, 30);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, false);
    // ADDS 10 + 20 = 30: N=0, Z=0, C=0, V=0
    assert_eq!(native_arm_flags, 0);

    // SUBS
    let bits = 0xEB020020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SUBS");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 30);
    interp.state.gpr.write_x(2, 10);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 20);
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);
    assert!(interp.state.pstate.flags.c); // C=1 for SUBS: no borrow (30 >= 10)
    assert!(!interp.state.pstate.flags.v);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 30, 10).unwrap();
    assert_eq!(native, 20);

    // Native with flags
    let bytes = lower_to_x86_64_with_flags(&decoded, false).unwrap();
    let (native_result, native_x86_flags) = common::win32::run_native_with_flags(&bytes, 30, 10, false).unwrap();
    assert_eq!(native_result, 20);
    let native_arm_flags = extract_arm_flags_from_x86(native_x86_flags, true);
    // SUBS 30 - 10 = 20: N=0, Z=0, C=1 (no borrow), V=0
    // 30 - 10 = 20, no borrow so C=1
    assert_eq!(native_arm_flags & 0b0010, 0b0010); // C=1
    assert_eq!(native_arm_flags & 0b0001, 0); // V=0
    assert_eq!(native_arm_flags & 0b1000, 0); // N=0
    assert_eq!(native_arm_flags & 0b0100, 0); // Z=0
}
