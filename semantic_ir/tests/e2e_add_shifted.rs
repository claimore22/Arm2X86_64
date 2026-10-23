use semantic_ir::{validate_instruction, BitWidth, Reg};
mod common;

use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;

/// End-to-end test: ADD (shifted register) decoded from AArch64 machine code,
/// lowered to semantic IR, validated, executed via IR interpreter, AND executed
/// as native x86-64 machine code.
///
/// Cross-check: IR interpreter result must equal native x86-64 execution result.
///
/// NOTE: QEMU-based cross-validation against native AArch64 execution is NOT
/// available on this Windows host. The IR interpreter serves as the
/// cross-validation reference. A true hardware/QEMU differential test should
/// be added when QEMU/aarch64-linux-gnu tooling is available.
#[test]
fn add_shifted_end_to_end() {
    // ADD X0, X1, X2, LSL #0 -> encoding 0x92020020
    let bits: u32 = 0x92020020;
    let decoded = decode_instruction(bits).expect("ADD X0, X1, X2, LSL #0 should decode");

    // Verify decode results
    assert_eq!(decoded.mnemonic, "ADD");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.shift, semantic_ir::primitives::ShiftKind::Lsl);
    assert_eq!(decoded.shift_amount, 0);
    assert_eq!(decoded.width, BitWidth::W64);

    // Lower to IR and validate
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("ADD IR should validate");

    // --- Execute via IR interpreter ---
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp
        .execute_instruction(&instruction)
        .expect("ADD should execute via IR");
    let ir_result = interp.state.gpr.read_x(0) as u64;
    assert_eq!(
        ir_result, 30,
        "IR interpreter: X0 should be X1 + X2 = 10 + 20 = 30"
    );

    // --- Lower to x86-64 machine code ---
    let bytes = lower_to_x86_64(&decoded).expect("ADD should lower to x86-64");
    assert_eq!(
        bytes,
        vec![0x48, 0x89, 0xC8, 0x48, 0x89, 0xD1, 0x48, 0xC1, 0xE1, 0x00, 0x48, 0x01, 0xC8, 0xC3],
        "x86-64 machine code must match expected encoding"
    );

    // --- Execute natively on x86-64 ---
    let native_result =
        common::win32::run_native(&bytes, 10, 20).expect("Native execution should succeed");
    // Independent AArch64 reference (differential validation oracle).
    let reference_result = common::aarch64_reference_add(10, 20, 0);
    assert_eq!(
        native_result, ir_result,
        "Native x86-64 ({}) must match IR interpreter ({})",
        native_result, ir_result
    );
    assert_eq!(
        native_result, reference_result,
        "Native x86-64 ({}) must match AArch64 reference ({})",
        native_result, reference_result
    );
    assert_eq!(
        native_result, 30,
        "Native x86-64: X0 should be X1 + X2 = 10 + 20 = 30"
    );
}

/// Multiple input cases including overflow and wrap-around.
#[test]
fn add_shifted_multiple_inputs() {
    // each tuple: (x1, x2)
    let cases: [(u64, u64); 5] = [
        (0, 0),                               // both zero
        (1, 0),                               // x2 = 0
        (0, 5),                               // x1 = 0
        (u64::MAX, 1),                        // overflow wraps to 0
        (u64::MAX / 2 + 1, u64::MAX / 2 + 1), // 2^63 + 2^63 = 0 mod 2^64
    ];

    for (x1, x2) in cases {
        // Independently calculated expected value (AArch64 reference semantics).
        let expected = common::aarch64_reference_add(x1, x2, 0);
        let bits: u32 = 0x92020020; // ADD X0, X1, X2, LSL #0
        let decoded =
            decode_instruction(bits).unwrap_or_else(|e| panic!("ADD decode failed: {:?}", e));
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction)
            .unwrap_or_else(|e| panic!("ADD IR validation failed: {:?}", e));

        // --- Lower to x86-64 machine code ---
        let bytes = lower_to_x86_64(&decoded)
            .unwrap_or_else(|e| panic!("ADD should lower to x86-64: {:?}", e));

        // IR interpreter
        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, x1);
        interp.state.gpr.write_x(2, x2);
        interp
            .execute_instruction(&instruction)
            .expect("ADD should validate via IR");
        let ir_result = interp.state.gpr.read_x(0) as u64;
        assert_eq!(
            ir_result, expected,
            "IR: x1={}, x2={}: expected {}, got {}",
            x1, x2, expected, ir_result
        );

        // Native x86-64
        let native_result = common::win32::run_native(&bytes, x1, x2)
            .unwrap_or_else(|e| panic!("Native execution failed: {:?}", e));
        assert_eq!(
            native_result, expected,
            "Native: x1={}, x2={}: expected {}, got {}",
            x1, x2, expected, native_result
        );
        assert_eq!(
            ir_result, native_result,
            "IR ({}) != Native ({}) for x1={}, x2={}",
            ir_result, native_result, x1, x2
        );
    }
}
