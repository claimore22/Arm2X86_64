mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn logical_not_variants() {
    // BIC X0, X1, X2, LSL #0 = 0x8A220020
    let bits = 0x8A220020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "BIC");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xF0F0);
    interp.state.gpr.write_x(2, 0x0FF0);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0) as u64, 0xF0F0 & !0x0FF0);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0xF0F0, 0x0FF0).unwrap();
    assert_eq!(native, 0xF0F0 & !0x0FF0);
}
