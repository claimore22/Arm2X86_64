mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn movz_x64() {
    let bits = 0xD2A24680u32; // MOVZ X0, #0x1234, lsl #16
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MOVZ");
    assert_eq!(decoded.imm, 0x1234);
    assert_eq!(decoded.shift_amount, 16);
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 0x12340000u64);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0, 0).unwrap();
    assert_eq!(native, 0x12340000u64);
}

#[test]
fn movn_x64() {
    let bits = 0x92A24680u32; // MOVN X0, #0x1234, lsl #16
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MOVN");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), !0x12340000u64);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0, 0).unwrap();
    assert_eq!(native, !0x12340000u64);
}

#[test]
fn movk_x64() {
    let bits = 0xF2A24680u32; // MOVK X0, #0x1234, lsl #16
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MOVK");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();

    // Set old X0 = 0xABCD0000EF012345
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, 0xABCD0000EF012345u64);
    interp.execute_instruction(&instruction).unwrap();
    let expected = (0xABCD0000EF012345u64 & !0x00000000FFFF0000u64) | 0x0000000012340000u64;
    assert_eq!(interp.state.gpr.read_x(0), expected);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native_rax(&bytes, 0xABCD0000EF012345u64).unwrap();
    assert_eq!(native, expected);
}

#[test]
fn movz_w32_zero_extends() {
    let bits = 0x529579A0u32; // MOVZ W0, #0xABCD
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MOVZ");
    assert_eq!(decoded.width, BitWidth::W32);
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, 0xFFFFFFFFFFFFFFFF); // old value should be cleared
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 0xABCDu64);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0, 0).unwrap();
    assert_eq!(native, 0xABCDu64);
}

#[test]
fn movk_w32_preserves() {
    let bits = 0x72A24680u32; // MOVK W0, #0x1234, lsl #16
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MOVK");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, 0xFFFFFFFFu64); // old lower 32 bits all ones
    interp.execute_instruction(&instruction).unwrap();
    // replace lane 16 with 0x1234, preserve lower 16
    let expected = (0xFFFFFFFFu64 & !0xFFFF0000u64) | 0x12340000u64;
    assert_eq!(interp.state.gpr.read_x(0), expected);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native_rax(&bytes, 0xFFFFFFFFu64).unwrap();
    assert_eq!(native, expected);
}
