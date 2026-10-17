/// Subtract shifted register, setting flags.
/// Stanford source: arm64_md_clean/subs_addsub_shift.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, UnaryOp, ValueType,
    Width, validate_instruction,
};

fn subs_shifted_instruction() -> Instruction {
    // This test instantiates the instruction at datasize = 64.
    //
    // The Stanford pseudocode uses symbolic `datasize`, but this particular
    // IR fixture is a concrete 64-bit instance.
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(Expr::Shift {
            kind: ShiftKind::Lsl,
            value: Box::new(Expr::ReadReg {
                reg: Reg::X(3),
                width: BitWidth::W64,
            }),
            amount: Box::new(Expr::Const {
                value: 4,
                ty: BitWidth::W32,
            }),
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Stanford:
    //
    // (result, nzcv) = AddWithCarry{datasize}(operand1, NOT(ShiftReg(m, shift_type, shift_amount)), '1');
    //
    // Tuple element 0 is the arithmetic result.
    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    let result = Expr::TupleElement {
        value: Box::new(add_with_carry.clone()),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Tuple element 1 is the complete 4-bit NZCV value.
    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry.clone()),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let mut semantics = vec![Stmt::WriteIndexedReg {
        index: Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        },
        value: result,
        width: datasize,
    }];

    // Stanford:
    //
    // N = nzcv<3>;
    // Z = nzcv<2>;
    // C = nzcv<1>;
    // V = nzcv<0>;
    //
    // BitTest converts the selected NZCV bit directly to Bool.
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
            mnemonic: "SUBS".to_string(),
            encoding: Some("Subtract shifted register, setting flags".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "subs_addsub_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn subs_shifted_register_validates() {
    let instruction = subs_shifted_instruction();

    validate_instruction(&instruction).expect("SUBS shifted register should validate");
}

#[test]
fn subs_shifted_register_writes_all_nzcv_flags() {
    let instruction = subs_shifted_instruction();

    assert_eq!(instruction.semantics.len(), 5);

    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteIndexedReg { .. }
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
                value: Expr::BitTest { .. },
            } if *flag == expected_flag
        ));
    }
}

#[test]
fn subs_shifted_register_extracts_nzcv_bits() {
    let instruction = subs_shifted_instruction();

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

#[test]
fn subs_shifted_register_uses_not_and_set_carry() {
    let instruction = subs_shifted_instruction();

    let Stmt::WriteIndexedReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected indexed register write");
    };

    let Expr::TupleElement {
        value: add_with_carry,
        index: 0,
..
    } = value
    else {
        panic!("expected AddWithCarry tuple result");
    };

    let Expr::AddWithCarry { right, carry, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    // Right operand should be Unary::Not wrapping the shifted register
    assert!(matches!(**right, Expr::Unary { op: UnaryOp::Not, .. }));

    // Carry should be Bool(true) for subtraction
    assert!(matches!(**carry, Expr::Bool(true)));
}
