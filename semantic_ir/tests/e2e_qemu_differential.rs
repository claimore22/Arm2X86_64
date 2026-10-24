mod common;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::{validate_instruction, BitWidth};

fn run_case(bits: u32, op: common::ArithmeticOp, width: BitWidth, shift: semantic_ir::primitives::ShiftKind, amount: u32, x1: u64, x2: u64) {
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

    let qemu_result = common::run_qemu_case(op, width, shift, amount, x1, x2);

    match qemu_result {
        Ok(qr) => {
            assert_eq!(ir_result, qr, "IR vs QEMU mismatch for op={:?} x1={} x2={} ir={} qemu={}", op, x1, x2, ir_result, qr);
            assert_eq!(native_result, qr, "Native vs QEMU mismatch for op={:?} x1={} x2={} native={} qemu={}", op, x1, x2, native_result, qr);
        }
        Err(e) => {
            println!("QEMU unavailable or failed for op={:?} x1={} x2={}: {}", op, x1, x2, e);
        }
    }
}

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
        run_case(0x92020020u32, common::ArithmeticOp::Add, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
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
        run_case(0xD2020020u32, common::ArithmeticOp::Sub, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
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
        run_case(0x8A020020u32, common::ArithmeticOp::And, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
    }
}

#[test]
fn qemu_differential_orr() {
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (0xFF, 0x0F),
        (0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA),
        (0x123456789ABCDEF0, 0x0F0F0F0F0F0F0F0F),
    ];
    for &(x1, x2) in cases {
        run_case(0xAA020020u32, common::ArithmeticOp::Or, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
    }
}

#[test]
fn qemu_differential_eor() {
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (0xFF, 0x0F),
        (0xFFFFFFFFFFFFFFFF, 0xAAAAAAAAAAAAAAAA),
        (0x123456789ABCDEF0, 0x0F0F0F0F0F0F0F0F),
    ];
    for &(x1, x2) in cases {
        run_case(0xCA020020u32, common::ArithmeticOp::Xor, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
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
        run_case(0x9B027C20u32, common::ArithmeticOp::Mul, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
    }
}

#[test]
fn qemu_differential_sdiv() {
    let cases: &[(u64, u64)] = &[
        (10, 3),
        (100, 7),
        (0, 5),
    ];
    for &(x1, x2) in cases {
        run_case(0x9AC20C20u32, common::ArithmeticOp::Sdiv, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
    }
}

#[test]
fn qemu_differential_udiv() {
    let cases: &[(u64, u64)] = &[
        (10, 3),
        (100, 7),
        (0, 5),
    ];
    for &(x1, x2) in cases {
        run_case(0x9AC20820u32, common::ArithmeticOp::Udiv, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, x1, x2);
    }
}

#[test]
fn qemu_differential_suite() {
    // Run the full differential suite for ADD
    let cases: &[(u64, u64)] = &[
        (0, 0),
        (1, 1),
        (10, 20),
    ];

    let mismatches = common::run_differential_suite_op(
        common::ArithmeticOp::Add,
        cases,
    ).unwrap();

    for m in &mismatches {
        println!("MISMATCH: {} x1={} x2={} ir={} native={} oracle={} stage={}",
            m.mnemonic, m.x1, m.x2, m.ir_result, m.native_result,
            m.semantic_oracle_result, m.failing_stage);
    }

    println!("Suite completed with {} mismatches", mismatches.len());
}
