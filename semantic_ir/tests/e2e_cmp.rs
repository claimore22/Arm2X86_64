mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn cmp_basic() {
    let bits = 0xEB02003Fu32; // CMP X1, X2 (Rd=XZR)
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SUBS");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, 10);
    interp.state.gpr.write_x(1, 10);
    interp.execute_instruction(&instruction).unwrap();
    // result discarded in XZR, but IR writes XZR (ignored). read_x(0) remains? WriteReg Xzr ignored, so read_x(0) unchanged (10).
    // Native returns computed result.
    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 10, 10).unwrap();
    assert_eq!(native, 0);
}
