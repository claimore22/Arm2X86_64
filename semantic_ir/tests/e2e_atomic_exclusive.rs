mod common;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth};

#[test]
fn exclusive_store_success() {
    let bits = 0xC85F7C22u32;
    let decoded = decode_instruction(bits).unwrap();
    let instr = lower_to_ir(&decoded);
    validate_instruction(&instr).unwrap();

    let bits_store = 0xC8007C22u32;
    let decoded_store = decode_instruction(bits_store).unwrap();
    let instr_store = lower_to_ir(&decoded_store);
    validate_instruction(&instr_store).unwrap();

    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x1000);
    interp.state.memory.ram[0x1000..0x1008].copy_from_slice(&42u64.to_le_bytes());
    interp.execute_instruction(&instr).unwrap();
    assert_eq!(interp.state.gpr.read_x(2), 42);
    interp.execute_instruction(&instr_store).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 0);
    let v = interp.state.memory.read_phys(0x1000, 8).unwrap();
    assert_eq!(v, 42);
}

#[test]
fn exclusive_store_failure_after_write() {
    let bits_load = 0xC85F7C22u32;
    let decoded_load = decode_instruction(bits_load).unwrap();
    let instr_load = lower_to_ir(&decoded_load);
    validate_instruction(&instr_load).unwrap();

    let bits_store = 0xC8007C22u32;
    let decoded_store = decode_instruction(bits_store).unwrap();
    let instr_store = lower_to_ir(&decoded_store);
    validate_instruction(&instr_store).unwrap();

    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x1000);
    interp.state.memory.ram[0x1000..0x1008].copy_from_slice(&42u64.to_le_bytes());
    interp.execute_instruction(&instr_load).unwrap();
    let _ = interp.state.memory.write_phys(0x1000, 8, 7);
    interp.execute_instruction(&instr_store).unwrap();
    assert_eq!(interp.state.gpr.read_x(0), 1);
}

#[test]
fn swp_exchanges() {
    let rs: u32 = 3;
    let rn: u32 = 1;
    let rt: u32 = 2;
    let bits = (3u32 << 30)
        | (0b111000u32 << 24)
        | (0u32 << 23)
        | (0u32 << 22)
        | (1u32 << 21)
        | (rs << 16)
        | (0b100000u32 << 10)
        | (rn << 5)
        | rt;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "SWP");
    let instr = lower_to_ir(&decoded);
    validate_instruction(&instr).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x2000);
    let oldval = 7u64;
    interp.state.memory.ram[0x2000..0x2008].copy_from_slice(&oldval.to_le_bytes());
    interp.state.gpr.write_x(3, 99);
    interp.execute_instruction(&instr).unwrap();
    assert_eq!(interp.state.gpr.read_x(2), oldval);
    assert_eq!(interp.state.memory.read_phys(0x2000, 8).unwrap(), 99);
}

#[test]
fn cas_success() {
    let rs: u32 = 6;
    let rn: u32 = 1;
    let rt: u32 = 3;
    let bits = (3u32 << 30)
        | (0b001000u32 << 24)
        | (1u32 << 23)
        | (0u32 << 22)
        | (1u32 << 21)
        | (rs << 16)
        | (0u32 << 15)
        | (0b11111u32 << 10)
        | (rn << 5)
        | rt;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "CAS");
    let instr = lower_to_ir(&decoded);
    validate_instruction(&instr).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x3000);
    let initial = 10u64;
    interp.state.memory.ram[0x3000..0x3008].copy_from_slice(&initial.to_le_bytes());
    interp.state.gpr.write_x(6, initial);
    interp.state.gpr.write_x(3, 99);
    interp.execute_instruction(&instr).unwrap();
    assert_eq!(interp.state.memory.read_phys(0x3000, 8).unwrap(), 99);
    assert_eq!(interp.state.gpr.read_x(6), initial);
}

#[test]
fn cas_failure_updates_compare() {
    let rs: u32 = 6;
    let rn: u32 = 1;
    let rt: u32 = 3;
    let bits = (3u32 << 30)
        | (0b001000u32 << 24)
        | (1u32 << 23)
        | (0u32 << 22)
        | (1u32 << 21)
        | (rs << 16)
        | (0u32 << 15)
        | (0b11111u32 << 10)
        | (rn << 5)
        | rt;
    let decoded = decode_instruction(bits).unwrap();
    let instr = lower_to_ir(&decoded);
    validate_instruction(&instr).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x4000);
    let initial = 10u64;
    interp.state.memory.ram[0x4000..0x4008].copy_from_slice(&initial.to_le_bytes());
    interp.state.gpr.write_x(6, 5);
    interp.state.gpr.write_x(3, 99);
    interp.execute_instruction(&instr).unwrap();
    assert_eq!(interp.state.memory.read_phys(0x4000, 8).unwrap(), initial);
    assert_eq!(interp.state.gpr.read_x(6), initial);
}

#[test]
fn casp_success() {
    let rs: u32 = 6;
    let rn: u32 = 1;
    let rt: u32 = 4;
    let sz = 0u32; // 32-bit pair (total 64 bits)
    let bits = (sz << 30)
        | (0b001000u32 << 24)
        | (1u32 << 23)
        | (0u32 << 22)
        | (1u32 << 21)
        | (rs << 16)
        | (0u32 << 15)
        | (0b11111u32 << 10)
        | (rn << 5)
        | rt;
    let decoded = decode_instruction(bits).unwrap();
    assert_eq!(decoded.mnemonic, "CASP");
    let instr = lower_to_ir(&decoded);
    validate_instruction(&instr).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x5000);
    let first = 10u32;
    let second = 20u32;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&first.to_le_bytes());
    bytes.extend_from_slice(&second.to_le_bytes());
    interp.state.memory.ram[0x5000..0x5008].copy_from_slice(&bytes);
    interp.state.gpr.write_x(6, first as u64);
    interp.state.gpr.write_x(7, second as u64);
    interp.state.gpr.write_x(4, 99);
    interp.state.gpr.write_x(5, 100);
    interp.execute_instruction(&instr).unwrap();
    let mut out = [0u8; 8];
    out.copy_from_slice(&interp.state.memory.ram[0x5000..0x5008]);
    let got_first = u32::from_le_bytes(out[0..4].try_into().unwrap()) as u64;
    let got_second = u32::from_le_bytes(out[4..8].try_into().unwrap()) as u64;
    assert_eq!(got_first, 99);
    assert_eq!(got_second, 100);
    assert_eq!(interp.state.gpr.read_x(6) as u32, first);
    assert_eq!(interp.state.gpr.read_x(7) as u32, second);
}

#[test]
fn casp_failure_updates_expected() {
    let rs: u32 = 6;
    let rn: u32 = 1;
    let rt: u32 = 4;
    let sz = 0u32;
    let bits = (sz << 30)
        | (0b001000u32 << 24)
        | (1u32 << 23)
        | (0u32 << 22)
        | (1u32 << 21)
        | (rs << 16)
        | (0u32 << 15)
        | (0b11111u32 << 10)
        | (rn << 5)
        | rt;
    let decoded = decode_instruction(bits).unwrap();
    let instr = lower_to_ir(&decoded);
    validate_instruction(&instr).unwrap();
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 0x6000);
    let first = 10u32;
    let second = 20u32;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&first.to_le_bytes());
    bytes.extend_from_slice(&second.to_le_bytes());
    interp.state.memory.ram[0x6000..0x6008].copy_from_slice(&bytes);
    interp.state.gpr.write_x(6, 5);
    interp.state.gpr.write_x(7, second as u64);
    interp.state.gpr.write_x(4, 99);
    interp.state.gpr.write_x(5, 100);
    interp.execute_instruction(&instr).unwrap();
    let mut out = [0u8; 8];
    out.copy_from_slice(&interp.state.memory.ram[0x6000..0x6008]);
    let got_first = u32::from_le_bytes(out[0..4].try_into().unwrap()) as u64;
    let got_second = u32::from_le_bytes(out[4..8].try_into().unwrap()) as u64;
    assert_eq!(got_first, first as u64);
    assert_eq!(got_second, second as u64);
    assert_eq!(interp.state.gpr.read_x(6) as u32, first);
    assert_eq!(interp.state.gpr.read_x(7) as u32, second);
}