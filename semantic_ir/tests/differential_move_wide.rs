mod common;
use common::{run_qemu_move_wide, wsl_qemu_available};
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn move_wide_qemu() {
    assert!(wsl_qemu_available());
    // MOVZ
    {
        let bits = 0xD2A24680u32;
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
        let mut interp = Interpreter::new();
        interp.execute_instruction(&instruction).unwrap();
        let ir = interp.state.gpr.read_x(0) as u64;
        let q = run_qemu_move_wide("MOVZ", BitWidth::W64, 0x1234, 16, 0).unwrap();
        assert_eq!(ir, q);
    }
    // MOVN
    {
        let bits = 0x92A24680u32;
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
        let mut interp = Interpreter::new();
        interp.execute_instruction(&instruction).unwrap();
        let ir = interp.state.gpr.read_x(0) as u64;
        let q = run_qemu_move_wide("MOVN", BitWidth::W64, 0x1234, 16, 0).unwrap();
        assert_eq!(ir, q);
    }
    // MOVK
    {
        let bits = 0xF2A24680u32;
        let decoded = decode_instruction(bits).unwrap();
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
        let old = 0xABCD0000EF012345u64;
        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(0, old);
        interp.execute_instruction(&instruction).unwrap();
        let ir = interp.state.gpr.read_x(0) as u64;
        let q = run_qemu_move_wide("MOVK", BitWidth::W64, 0x1234, 16, old).unwrap();
        assert_eq!(ir, q);
    }
}
