use semantic_ir::decode::{decode_instruction, lower_to_ir, DecodeError};
use semantic_ir::{validate_instruction, BitWidth, Reg};

const SUB_X0_X1_X2_LSL0_64: u32 = 0xD2020020;
const SUB_W0_W1_W2_LSL0_32: u32 = 0x52020020;

#[test]
fn decode_sub_shifted_register_64() {
    let decoded = decode_instruction(SUB_X0_X1_X2_LSL0_64).expect("SUB should decode");
    assert_eq!(decoded.mnemonic, "SUB");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn decode_sub_shifted_register_32() {
    let decoded = decode_instruction(SUB_W0_W1_W2_LSL0_32).expect("SUB should decode");
    assert_eq!(decoded.mnemonic, "SUB");
    assert_eq!(decoded.rd, Reg::W(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.width, BitWidth::W32);
}

#[test]
fn decoded_sub_validates() {
    let decoded = decode_instruction(SUB_X0_X1_X2_LSL0_64).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("SUB IR should validate");
}

#[test]
fn decoded_sub_interprets_correctly() {
    let decoded = decode_instruction(SUB_X0_X1_X2_LSL0_64).unwrap();
    let instruction = lower_to_ir(&decoded);
    let mut interp = semantic_ir::interpreter::Interpreter::new();
    interp.state.gpr.write_x(1, 30);
    interp.state.gpr.write_x(2, 10);
    interp.execute_instruction(&instruction).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 20);
}
