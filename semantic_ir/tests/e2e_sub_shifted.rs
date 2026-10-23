mod common;

use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth, Reg};

#[test]
fn sub_shifted_end_to_end() {
    let bits: u32 = 0xD2020020; // SUB X0, X1, X2, LSL #0
    let decoded = decode_instruction(bits).expect("SUB X0, X1, X2, LSL #0 should decode");
    assert_eq!(decoded.mnemonic, "SUB");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.shift_amount, 0);
    assert_eq!(decoded.width, BitWidth::W64);

    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("SUB IR should validate");

    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 30);
    interp.state.gpr.write_x(2, 10);
    interp
        .execute_instruction(&instruction)
        .expect("SUB should execute via IR");
    let ir_result = interp.state.gpr.read_x(0) as u64;
    assert_eq!(ir_result, 20);

    let bytes = lower_to_x86_64(&decoded).expect("SUB should lower to x86-64");
    assert_eq!(
        bytes,
        vec![0x48, 0x89, 0xC8, 0x48, 0x89, 0xD1, 0x48, 0xC1, 0xE1, 0x00, 0x48, 0x29, 0xC8, 0xC3],
        "x86-64 machine code must match expected encoding"
    );

    let native_result =
        common::win32::run_native(&bytes, 30, 10).expect("Native execution should succeed");
    let reference_result = common::aarch64_reference_sub(30, 10, 0);
    assert_eq!(native_result, ir_result);
    assert_eq!(native_result, reference_result);
    assert_eq!(native_result, 20);
}

#[test]
fn sub_shifted_multiple_inputs() {
    let cases: [(u64, u64); 5] = [
        (0, 0),
        (5, 3),
        (0, 1), // underflow wraps
        (u64::MIN, 0),
        (u64::MAX, u64::MAX),
    ];
    for (x1, x2) in cases {
        let bits: u32 = 0xD2020020;
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();

        let bytes = lower_to_x86_64(&decoded).unwrap();

        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, x1);
        interp.state.gpr.write_x(2, x2);
        interp.execute_instruction(&instruction).unwrap();
        let ir_result = interp.state.gpr.read_x(0) as u64;

        let expected = common::aarch64_reference_sub(x1, x2, 0);
        assert_eq!(ir_result, expected);

        let native_result = common::win32::run_native(&bytes, x1, x2).unwrap();
        assert_eq!(native_result, expected);
        assert_eq!(ir_result, native_result);
    }
}
