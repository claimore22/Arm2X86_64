use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn csel_basic() {
    // CSEL X0, X1, X2, EQ (encoding 0x9A820020)
    let bits = 0x9A820020u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "CSEL");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();

    // EQ true (Z=1)
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp.state.pstate.flags.z = true;
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 10);

    // EQ false
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp.state.pstate.flags.z = false;
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 20);
}

#[test]
fn csinc_csinv_csneg() {
    let cases = [
        (0x9A820420u32, "CSINC"),
        (0xDA820020u32, "CSINV"),
        (0xDA820420u32, "CSNEG"),
    ];
    for (bits, name) in cases {
        let decoded = decode_instruction(bits).unwrap();
        assert_eq!(decoded.mnemonic, name);
        let instruction = lower_to_ir(&decoded);
        validate_instruction(&instruction).unwrap();
    }
}
