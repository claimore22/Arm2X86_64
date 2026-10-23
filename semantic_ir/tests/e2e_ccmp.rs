use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn ccmp_basic() {
    let bits = 0xFA410000u32; // CCMP X0, X1, #0, EQ
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "CCMP");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();

    // condition false (Z=0) -> flags set to nzcv=0
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, 10);
    interp.state.gpr.write_x(1, 20);
    interp.state.pstate.flags.z = false;
    interp.execute_instruction(&instruction).unwrap();
    assert!(!interp.state.pstate.flags.n);
    assert!(!interp.state.pstate.flags.z);

    // condition true (Z=1) -> compare 10 vs 20
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, 10);
    interp.state.gpr.write_x(1, 20);
    interp.state.pstate.flags.z = true;
    interp.execute_instruction(&instruction).unwrap();
    assert!(interp.state.pstate.flags.n); // 10-20 negative
}
