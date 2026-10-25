mod common;
use common::{run_qemu_case, wsl_qemu_available};
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth, Reg};

#[test]
fn logical_qemu() {
    assert!(wsl_qemu_available());
    for (bits, op) in [
        (0x94020020u32, common::ArithmeticOp::And),
        (0xB4020020u32, common::ArithmeticOp::Or),
        (0xD4020020u32, common::ArithmeticOp::Xor),
        (0xF4020020u32, common::ArithmeticOp::Ands),
    ] {
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, 0xF0F0);
        interp.state.gpr.write_x(2, 0x0FF0);
        interp.execute_instruction(&instruction).unwrap();
        let ir = interp.state.gpr.read_x(0) as u64;
        let q = run_qemu_case(op, BitWidth::W64, semantic_ir::primitives::ShiftKind::Lsl, 0, 0xF0F0, 0x0FF0).unwrap();
        assert_eq!(ir, q, "{:?}", op);
    }
}

