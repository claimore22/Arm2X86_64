mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn addi_subi() {
    for (bits, opname, expected) in [(0x91000420u32, "ADDI", 11u64), (0xD1000420u32, "SUBI", 9u64)] {
        let decoded = decode_instruction(bits).unwrap();
        assert_eq!(decoded.mnemonic, opname);
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(1, 10);
        interp.execute_instruction(&instruction).unwrap();
        assert_eq!(interp.state.gpr.read_x(0), expected);
        let bytes = lower_to_x86_64(&decoded).unwrap();
        let native = common::win32::run_native(&bytes, 10, 0).unwrap();
        assert_eq!(native, expected);
    }
}
