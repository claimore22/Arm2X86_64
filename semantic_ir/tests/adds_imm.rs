/// ADDS (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/adds_addsub_imm.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn adds_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let n_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };

    let operand1 = Expr::Select {
        condition: Box::new(n_is_sp),
        when_true: Box::new(Expr::ReadSP {
            width: BitWidth::W64,
        }),
        when_false: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let imm_bits = BitWidth::new(24).unwrap();
    let operand2 = Expr::ZeroExtend {
        value: Box::new(Expr::Const {
            value: 0x123456,
            ty: imm_bits,
        }),
        to: BitWidth::W64,
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(false)),
        width: BitWidth::W64,
    };

    let result = Expr::TupleElement {
        value: Box::new(add_with_carry.clone()),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let mut semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    for (flag, bit) in [(Flag::N, 3), (Flag::Z, 2), (Flag::C, 1), (Flag::V, 0)] {
        semantics.push(Stmt::WriteFlag {
            flag,
            value: Expr::BitTest {
                value: Box::new(nzcv.clone()),
                bit,
            },
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: "ADDS".to_string(),
            encoding: Some("Add immediate, setting flags".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adds_addsub_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adds_imm_validates() {
    let instruction = adds_imm_instruction();

    validate_instruction(&instruction).expect("ADDS immediate should validate");
}

#[test]
fn adds_imm_writes_result_and_flags() {
    let instruction = adds_imm_instruction();

    assert_eq!(instruction.semantics.len(), 5);

    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));

    for (statement, expected_flag) in [
        (&instruction.semantics[1], Flag::N),
        (&instruction.semantics[2], Flag::Z),
        (&instruction.semantics[3], Flag::C),
        (&instruction.semantics[4], Flag::V),
    ] {
        assert!(matches!(
            statement,
            Stmt::WriteFlag {
                flag,
..
            } if *flag == expected_flag
        ));
    }
}

#[test]
fn adds_imm_extracts_nzcv_correctly() {
    let instruction = adds_imm_instruction();

    for (statement, expected_bit, expected_flag) in [
        (&instruction.semantics[1], 3, Flag::N),
        (&instruction.semantics[2], 2, Flag::Z),
        (&instruction.semantics[3], 1, Flag::C),
        (&instruction.semantics[4], 0, Flag::V),
    ] {
        let Stmt::WriteFlag { flag, value } = statement else {
            panic!("expected WriteFlag");
        };

        assert_eq!(*flag, expected_flag);

        let Expr::BitTest {
            value: nzcv,
            bit,
        } = value
        else {
            panic!("expected BitTest");
        };

        assert_eq!(*bit, expected_bit);

        assert!(matches!(
            nzcv.as_ref(),
            Expr::TupleElement {
                index: 1,
                ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
..
            }
        ));
    }
}
