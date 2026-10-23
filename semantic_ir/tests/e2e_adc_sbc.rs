mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn adc_sbc() {
    // ADC X0, X1, X2 = 0x9A020020
    let decoded = decode_instruction(0x9A020020u32).unwrap();
    assert_eq!(decoded.mnemonic, "ADC");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp.state.pstate.flags.c = false;
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 30); // 10+20+0

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 10, 20).unwrap();
    assert_eq!(native, 30);

    // SBC
    let decoded = decode_instruction(0xDA020020u32).unwrap();
    assert_eq!(decoded.mnemonic, "SBC");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 30);
    interp.state.gpr.write_x(2, 10);
    interp.state.pstate.flags.c = false; // borrow
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 19); // 30-10-1

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 30, 10).unwrap();
    assert_eq!(native, 19);
}
