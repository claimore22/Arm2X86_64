mod common;

use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth, Reg};

#[test]
fn and_shifted_end_to_end() {
    let bits: u32 = 0x8A020020; // AND X0, X1, X2, LSL #0
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "AND");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();

    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xF0F0);
    interp.state.gpr.write_x(2, 0x0FF0);
    interp.execute_instruction(&instruction).unwrap();
    let ir_result = interp.state.gpr.read_x(0) as u64;
    assert_eq!(ir_result, 0x00F0);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0xF0F0, 0x0FF0).unwrap();
    assert_eq!(native, ir_result);
    assert_eq!(native, 0x00F0);
}

#[test]
fn orr_eor_ands_basic() {
    for (bits, op) in [(0xAA020020u32,"ORR"),(0xCA020020u32,"EOR"),(0xEA020020u32,"ANDS")] {
        let decoded = decode_instruction(bits).unwrap();
        assert_eq!(decoded.mnemonic, op);
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, 0xF0);
        interp.state.gpr.write_x(2, 0x0F);
        interp.execute_instruction(&instruction).unwrap();
        let r = interp.state.gpr.read_x(0) as u64;
        let expected = match op {
            "ORR" => 0xFFu64,
            "EOR" => 0xFFu64,
            "ANDS" => 0x00u64,
            _ => 0,
        };
        assert_eq!(r, expected, "{}", op);
        if op == "ANDS" {
            assert!(!interp.state.pstate.flags.n);
            assert!(interp.state.pstate.flags.z);
        }
    }
}
