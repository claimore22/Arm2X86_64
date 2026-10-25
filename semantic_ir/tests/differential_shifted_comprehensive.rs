mod common;

use common::{run_qemu_case, wsl_qemu_available};
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::{validate_instruction, BitWidth, Reg, ShiftKind};

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
}

impl Op {
    fn mnemonic(self) -> &'static str {
        match self {
            Op::Add => "ADD",
            Op::Sub => "SUB",
        }
    }
    fn op0(self) -> u32 {
        match self {
            Op::Add => 0,
            Op::Sub => 2,
        }
    }
    fn reference(self, xn: u64, xm: u64, shift: ShiftKind, amt: u32, width: BitWidth) -> u64 {
        let mask = match width {
            BitWidth::W32 => 0xFFFFFFFFu64,
            _ => u64::MAX,
        };
        let xn = xn & mask;
        let xm = xm & mask;
        let shifted = match shift {
            ShiftKind::Lsl => xm.wrapping_shl(amt),
            ShiftKind::Lsr => xm >> amt,
            ShiftKind::Asr => {
                if width == BitWidth::W32 {
                    ((xm as i32) >> amt) as u32 as u64
                } else {
                    ((xm as i64) >> amt) as u64
                }
            }
            _ => xm,
        } & mask;
        let res = match self {
            Op::Add => xn.wrapping_add(shifted),
            Op::Sub => xn.wrapping_sub(shifted),
        };
        res & mask
    }
    fn common_op(self) -> common::ArithmeticOp {
        match self {
            Op::Add => common::ArithmeticOp::Add,
            Op::Sub => common::ArithmeticOp::Sub,
        }
    }
}

fn encode(op: Op, width: BitWidth, shift: ShiftKind, amount: u32, rd: u8, rn: u8, rm: u8) -> u32 {
    let sf = match width {
        BitWidth::W64 => 1u32,
        _ => 0u32,
    };
    let shift_bits = match shift {
        ShiftKind::Lsl => 0u32,
        ShiftKind::Lsr => 1u32,
        ShiftKind::Asr => 2u32,
        _ => 0u32,
    };
    let mut bits = 0u32;
    bits |= sf << 31;
    bits |= op.op0() << 29;
    bits |= 1 << 28;
    bits |= 1 << 25;
    bits |= shift_bits << 21;  // Fixed: shift field is bits 22-21, not 23-22
    bits |= (rm as u32) << 16;
    bits |= amount << 10;
    bits |= (rn as u32) << 5;
    bits |= rd as u32;
    bits
}

#[test]
fn qemu_shifted_comprehensive() {
    assert!(wsl_qemu_available(), "WSL QEMU must be available");

    let shifts = [ShiftKind::Lsl, ShiftKind::Lsr, ShiftKind::Asr];
    let widths = [BitWidth::W64, BitWidth::W32];
    let amounts: &[u32] = &[0, 1, 2, 31];
    // For 64-bit also test 63.
    let mut executed = 0usize;

    for &op in &[Op::Add, Op::Sub] {
        for &width in &widths {
            let max_amount = match width {
                BitWidth::W64 => 63u32,
                _ => 31u32,
            };
            let amt_list: Vec<u32> = if width == BitWidth::W64 {
                vec![0, 1, 2, 31, 63]
            } else {
                vec![0, 1, 2, 31]
            };
            for &shift in &shifts {
                // skip asr/lsl with amount 0? all valid.
                for &amount in &amt_list {
                    // Case 1: normal registers.
                    {
                        let bits = encode(op, width, shift, amount, 0, 1, 2);
                        let decoded = decode_instruction(bits).unwrap_or_else(|e| panic!("decode {:?}: {:?}", bits, e));
                        assert_eq!(decoded.mnemonic, op.mnemonic());
                        let instruction = lower_to_ir(&decoded);
                        validate_instruction(&instruction).unwrap();

                        // IR
                        let (x1, x2) = (0x1234_5678_9ABC_DEF0u64, 0x0FED_CBA9_8765_4321u64);
                        let (x1, x2) = match width {
                            BitWidth::W32 => (x1 & 0xFFFFFFFF, x2 & 0xFFFFFFFF),
                            _ => (x1, x2),
                        };
                        let mut interp = Interpreter::new();
                        interp.state.gpr.write_x(1, x1);
                        interp.state.gpr.write_x(2, x2);
                        interp.execute_instruction(&instruction).unwrap();
                        let ir_result = interp.state.gpr.read_x(0) as u64 & match width {
                            BitWidth::W32 => 0xFFFFFFFF,
                            _ => u64::MAX,
                        };

                        // QEMU
                        let q = run_qemu_case(op.common_op(), width, shift, amount, x1, x2).unwrap();
                        let expected = op.reference(x1, x2, shift, amount, width);
                        if ir_result != expected || q != expected {
                            eprintln!("MISMATCH op={:?} width={:?} shift={:?} amt={} ir={:x} q={:x} exp={:x}", op, width, shift, amount, ir_result, q, expected);
                        }
                        assert_eq!(ir_result, expected, "IR mismatch");
                        assert_eq!(q, expected, "QEMU mismatch");
                        executed += 1;
                    }

                    // Case 2: zero register as rn (XZR/WZR) -> 0 op (rm shifted)
                    {
                        let bits = encode(op, width, shift, amount, 0, 31, 2);
                        let decoded = decode_instruction(bits).unwrap();
                        let instruction = lower_to_ir(&decoded);
                        validate_instruction(&instruction).unwrap();
                        let x2 = match width {
                            BitWidth::W32 => 42u64,
                            _ => 0x1234u64,
                        };
                        let mut interp = Interpreter::new();
                        interp.state.gpr.write_x(2, x2);
                        interp.execute_instruction(&instruction).unwrap();
                        let ir_result = interp.state.gpr.read_x(0) as u64 & match width {
                            BitWidth::W32 => 0xFFFFFFFF,
                            _ => u64::MAX,
                        };
                        let q = run_qemu_case(op.common_op(), width, shift, amount, 0, x2).unwrap();
                        let expected = op.reference(0, x2, shift, amount, width);
                        assert_eq!(ir_result, expected);
                        assert_eq!(q, expected);
                        executed += 1;
                    }

                    // Case 3: zero register as rm -> xn op 0
                    {
                        let bits = encode(op, width, shift, amount, 0, 1, 31);
                        let decoded = decode_instruction(bits).unwrap();
                        let instruction = lower_to_ir(&decoded);
                        validate_instruction(&instruction).unwrap();
                        let x1 = match width {
                            BitWidth::W32 => 7u64,
                            _ => 0x9999u64,
                        };
                        let mut interp = Interpreter::new();
                        interp.state.gpr.write_x(1, x1);
                        interp.execute_instruction(&instruction).unwrap();
                        let ir_result = interp.state.gpr.read_x(0) as u64 & match width {
                            BitWidth::W32 => 0xFFFFFFFF,
                            _ => u64::MAX,
                        };
                        let q = run_qemu_case(op.common_op(), width, shift, amount, x1, 0).unwrap();
                        let expected = op.reference(x1, 0, shift, amount, width);
                        assert_eq!(ir_result, expected);
                        assert_eq!(q, expected);
                        executed += 1;
                    }
                }
            }
        }
    }

    eprintln!("QEMU executed {} cases", executed);
    assert!(executed > 100);
}
