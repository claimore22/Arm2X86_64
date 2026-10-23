mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn mul_x64() {
    let bits = 0x9B027C20u32; // MUL X0, X1, X2
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MUL");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 6);
    interp.state.gpr.write_x(2, 7);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 42);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 6, 7).unwrap();
    assert_eq!(native, 42);
}

#[test]
fn mul_w32() {
    let bits = 0x1B027C20u32; // MUL W0, W1, W2
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "MUL");
    assert_eq!(decoded.width, BitWidth::W32);
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 6);
    interp.state.gpr.write_x(2, 7);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 42);
}
