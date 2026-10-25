mod common;
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn logical_imm_x64() {
    // ANDI X0, X1, #0xFFFF0000FFFF0000 (N=1, imms=0x1F, immr=0)
    // Encoding: sf=1, opc=00, N=1, fixed=00100100, immr=0, imms=0x1F, Rn=1, Rd=0
    // = 0x92407C20
    let bits = 0x92407C20u32; // ANDI X0, X1, #0xFFFF0000FFFF0000
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "ANDI");
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0xFFFFFFFFFFFFFFFFu64);
    interp.execute_instruction(&instruction).unwrap();
    let ir = interp.state.gpr.read_x(0) as u64;
    // compute mask
    let (imm, _) = semantic_ir::shared::decode_bit_masks::<u8,u64>(decoded.n==1, decoded.imm as u8, decoded.shift_amount as u8, true);
    assert_eq!(ir, imm);

    let bytes = lower_to_x86_64(&decoded).unwrap();
    let native = common::win32::run_native(&bytes, 0xFFFFFFFFFFFFFFFFu64, 0).unwrap();
    assert_eq!(native, imm);
}
