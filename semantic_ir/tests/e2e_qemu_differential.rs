mod common;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn qemu_differential_add() {
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (1, 2),
        (10, 20),
        (0xFFFFFFFFFFFFFFFF, 1),
        (0x7FFFFFFFFFFFFFFF, 1),
        (0x8000000000000000, 0x8000000000000000),
    ];

    for &(x1, x2) in cases {
        let bits = 0x8B020020u32; // ADD X0, X1, X2, LSL #0
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();

        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, x1);
        interp.state.gpr.write_x(2, x2);
        interp.execute_instruction(&instruction).unwrap();
        let ir_result = interp.state.gpr.read_x(0);

        let bytes = lower_to_x86_64(&decoded).unwrap();
        let native_result = common::win32::run_native(&bytes, x1, x2).unwrap();

        let qemu_result = common::run_qemu_case(
            common::ArithmeticOp::Add,
            BitWidth::W64,
            semantic_ir::primitives::ShiftKind::Lsl,
            0,
            x1,
            x2,
        );

        match qemu_result {
            Ok(qr) => {
                assert_eq!(ir_result, qr, "IR vs QEMU mismatch for {} + {}", x1, x2);
                assert_eq!(native_result, qr, "Native vs QEMU mismatch for {} + {}", x1, x2);
            }
            Err(e) => {
                println!("QEMU unavailable or failed: {}", e);
            }
        }
    }
}

#[test]
fn qemu_differential_sub() {
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (10, 3),
        (3, 10),
        (0, 1),
        (1, 0),
    ];

    for &(x1, x2) in cases {
        let bits = 0xCB020020u32; // SUB X0, X1, X2, LSL #0
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();

        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, x1);
        interp.state.gpr.write_x(2, x2);
        interp.execute_instruction(&instruction).unwrap();
        let ir_result = interp.state.gpr.read_x(0);

        let bytes = lower_to_x86_64(&decoded).unwrap();
        let native_result = common::win32::run_native(&bytes, x1, x2).unwrap();

        let qemu_result = common::run_qemu_case(
            common::ArithmeticOp::Sub,
            BitWidth::W64,
            semantic_ir::primitives::ShiftKind::Lsl,
            0,
            x1,
            x2,
        );

        match qemu_result {
            Ok(qr) => {
                assert_eq!(ir_result, qr, "IR vs QEMU mismatch for {} - {}", x1, x2);
                assert_eq!(native_result, qr, "Native vs QEMU mismatch for {} - {}", x1, x2);
            }
            Err(e) => {
                println!("QEMU unavailable or failed: {}", e);
            }
        }
    }
}

#[test]
fn qemu_differential_and() {
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (0xFF, 0x0F),
        (0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA),
        (0x123456789ABCDEF0, 0x0F0F0F0F0F0F0F0F),
    ];

    for &(x1, x2) in cases {
        let bits = 0x8A020020u32; // AND X0, X1, X2, LSL #0
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();

        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, x1);
        interp.state.gpr.write_x(2, x2);
        interp.execute_instruction(&instruction).unwrap();
        let ir_result = interp.state.gpr.read_x(0);

        let bytes = lower_to_x86_64(&decoded).unwrap();
        let native_result = common::win32::run_native(&bytes, x1, x2).unwrap();

        let qemu_result = common::run_qemu_case(
            common::ArithmeticOp::And,
            BitWidth::W64,
            semantic_ir::primitives::ShiftKind::Lsl,
            0,
            x1,
            x2,
        );

        match qemu_result {
            Ok(qr) => {
                assert_eq!(ir_result, qr, "IR vs QEMU mismatch for {} & {}", x1, x2);
                assert_eq!(native_result, qr, "Native vs QEMU mismatch for {} & {}", x1, x2);
            }
            Err(e) => {
                println!("QEMU unavailable or failed: {}", e);
            }
        }
    }
}

#[test]
fn qemu_differential_mul() {
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (1, 2),
        (10, 20),
        (0xFFFFFFFFFFFFFFFF, 2),
        (12345, 67890),
    ];

    for &(x1, x2) in cases {
        let bits = 0x9B027C20u32; // MUL X0, X1, X2
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();

        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, x1);
        interp.state.gpr.write_x(2, x2);
        interp.execute_instruction(&instruction).unwrap();
        let ir_result = interp.state.gpr.read_x(0);

        let bytes = lower_to_x86_64(&decoded).unwrap();
        let native_result = common::win32::run_native(&bytes, x1, x2).unwrap();

        let qemu_result = common::run_qemu_case(
            common::ArithmeticOp::Mul,
            BitWidth::W64,
            semantic_ir::primitives::ShiftKind::Lsl,
            0,
            x1,
            x2,
        );

        match qemu_result {
            Ok(qr) => {
                assert_eq!(ir_result, qr, "IR vs QEMU mismatch for {} * {}", x1, x2);
                assert_eq!(native_result, qr, "Native vs QEMU mismatch for {} * {}", x1, x2);
            }
            Err(e) => {
                println!("QEMU unavailable or failed: {}", e);
            }
        }
    }
}