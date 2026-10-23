mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn bfm_x64() {
    // BFM X0, X1, #4, #7  (encoding 0xB3441C20)
    let bits = 0xB3441C20u32;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "BFM");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();

    let src = 0x123456789ABCDEF0u64;
    let old = 0x0FEDCBA987654321u64;
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(0, old);
    interp.state.gpr.write_x(1, src);
    interp.execute_instruction(&instruction).unwrap();
    let ir = interp.state.gpr.read_x(0) as u64;

    // expected via IR formula: (old & ~tmask) | (bot & tmask)
    // We'll just compare native to ir.
    let bytes = lower_to_x86_64(&decoded).unwrap();
    // pass src as x1, old as x2
    let native = common::win32::run_native(&bytes, src, old).unwrap();
    assert_eq!(native, ir);
}

#[test]
fn ubfm_x64() {
    let bits = 0xD3441C20u32; // UBFM X0, X1, #4, #7
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "UBFM");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x123456789ABCDEF0u64);
    interp.execute_instruction(&instruction).unwrap();
    let ir = interp.state.gpr.read_x(0) as u64;
    // extract 4 bits at pos4
    let expected = (0x123456789ABCDEF0u64 >> 4) & 0xF;
    assert_eq!(ir, expected);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0, 0).unwrap(); // src passed? run_native sets RCX=x1? Actually x1 passed as first arg. But codegen assumes src in RCX. run_native(x1=src? signature run_native(bytes, x1, x2) sets RCX=x1. We passed 0 for x1! Need pass src as x1.
    // redo correctly:
    let native = common::win32::run_native(&bytes, 0x123456789ABCDEF0u64, 0).unwrap();
    assert_eq!(native, expected);
}

#[test]
fn sbfm_x64() {
    let bits = 0x93441C20u32; // SBFM X0, X1, #4, #7
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SBFM");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x123456789ABCDEF0u64);
    interp.execute_instruction(&instruction).unwrap();
    let ir = interp.state.gpr.read_x(0) as u64;
    // extract 4 bits at pos4 and sign-extend
    let field = (0x123456789ABCDEF0u64 >> 4) & 0xF;
    let expected = if field & 0x8 != 0 { field | !0xFu64 } else { field };
    assert_eq!(ir, expected);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0x123456789ABCDEF0u64, 0).unwrap();
    assert_eq!(native, expected);
}
