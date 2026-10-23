use semantic_ir::decode::{decode_instruction, lower_to_ir, DecodeError};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{
    validate_instruction, BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    ValueType, Width,
};

/// ADD X0, X1, X2, LSL #0 → 0x92020020
const ADD_X0_X1_X2_LSL0_64: u32 = 0x92020020;

/// ADD W0, W1, W2, LSL #0 → 0x12020020
const ADD_W0_W1_W2_LSL0_32: u32 = 0x12020020;

/// Invalid: shift=11 (reserved) is not supported
const INVALID_SHIFT_64: u32 = 0x92C20020;

/// Invalid: reserved op0 bits
const INVALID_ENCODING_64: u32 = 0xF2020020;

#[test]
fn decode_add_shifted_register_64() {
    let decoded =
        decode_instruction(ADD_X0_X1_X2_LSL0_64).expect("ADD X0, X1, X2, LSL #0 should decode");
    assert_eq!(decoded.mnemonic, "ADD");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.shift, semantic_ir::ShiftKind::Lsl);
    assert_eq!(decoded.shift_amount, 0);
    assert_eq!(decoded.width, BitWidth::W64);
}

#[test]
fn decode_add_shifted_register_32() {
    let decoded =
        decode_instruction(ADD_W0_W1_W2_LSL0_32).expect("ADD W0, W1, W2, LSL #0 should decode");
    assert_eq!(decoded.mnemonic, "ADD");
    assert_eq!(decoded.rd, Reg::W(0));
    assert_eq!(decoded.rn, Reg::W(1));
    assert_eq!(decoded.rm, Reg::W(2));
    assert_eq!(decoded.shift, semantic_ir::ShiftKind::Lsl);
    assert_eq!(decoded.shift_amount, 0);
    assert_eq!(decoded.width, BitWidth::W32);
}

#[test]
fn decode_invalid_shift() {
    let result = decode_instruction(INVALID_SHIFT_64);
    assert!(matches!(result, Err(DecodeError::InvalidShift)));
}

#[test]
fn decode_invalid_encoding() {
    let result = decode_instruction(INVALID_ENCODING_64);
    assert!(matches!(result, Err(DecodeError::UndefinedInstruction)));
}

#[test]
fn decoded_add_validates() {
    let decoded = decode_instruction(ADD_X0_X1_X2_LSL0_64).unwrap();
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("ADD (shifted register) IR should validate");
}

#[test]
fn decoded_add_interprets_correctly() {
    let decoded = decode_instruction(ADD_X0_X1_X2_LSL0_64).unwrap();
    let instruction = lower_to_ir(&decoded);

    let mut interp = semantic_ir::interpreter::Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);

    interp
        .execute_instruction(&instruction)
        .expect("ADD should execute");
    assert_eq!(interp.state.gpr.read_x(0), 30);
}

#[test]
fn decoded_add_shift_validates() {
    // ADD X0, X1, X2, LSL #2
    let bits: u32 = 0x92020020 | (2 << 10);
    let decoded = decode_instruction(bits).expect("ADD X0, X1, X2, LSL #2 should decode");
    assert_eq!(decoded.shift_amount, 2);

    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("ADD with shift should validate");
}

#[test]
fn decoded_add_shift_interprets_correctly() {
    // ADD X0, X1, X2, LSL #2 → X0 = X1 + (X2 << 2)
    let bits: u32 = 0x92020020 | (2 << 10);
    let decoded = decode_instruction(bits).unwrap();
    let instruction = lower_to_ir(&decoded);

    let mut interp = semantic_ir::interpreter::Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 5); // X2 << 2 = 20

    interp
        .execute_instruction(&instruction)
        .expect("ADD should execute");
    assert_eq!(interp.state.gpr.read_x(0), 30); // 10 + 20 = 30
}

#[test]
fn decoded_add_wzr_reads_zero() {
    // ADD W0, WZR, W2 → 0x120203E0 (Rn=31=WZR)
    let bits: u32 = 0x120203E0;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.rn, Reg::Wzr);
    assert_eq!(decoded.rm, Reg::W(2));

    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("ADD with WZR should validate");

    let mut interp = semantic_ir::interpreter::Interpreter::new();
    interp.state.gpr.write_x(2, 42);
    interp
        .execute_instruction(&instruction)
        .expect("ADD with WZR should execute");
    assert_eq!(interp.state.gpr.read_x(0), 42); // 0 + 42 = 42
}
