/// CMP (shifted register) is an alias of SUBS (shifted register).
/// Stanford source: arm64_md_clean/cmp_subs_addsub_shift.md
/// It subtracts shifted register from register, updates flags, but discards the result.
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, UnaryOp,
    ValueType, Width, validate_instruction,
};

fn cmp_shifted_instruction() -> Instruction {
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

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    // CMP discards the result (no register write), only updates flags
    let mut semantics = Vec::new();
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
            mnemonic: "CMP".to_string(),
            encoding: Some("Compare (shifted register, alias of SUBS)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmp_subs_addsub_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmp_shifted_validates() {
    let instruction = cmp_shifted_instruction();

    validate_instruction(&instruction).expect("CMP shifted register should validate");
}

#[test]
fn cmp_shifted_writes_flags_only() {
    let instruction = cmp_shifted_instruction();

    // Should only write flags, no register write
    assert_eq!(instruction.semantics.len(), 4);
    for (statement, expected_flag) in [
        (&instruction.semantics[0], Flag::N),
        (&instruction.semantics[1], Flag::Z),
        (&instruction.semantics[2], Flag::C),
        (&instruction.semantics[3], Flag::V),
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
fn cmp_shifted_uses_not_and_set_carry() {
    let instruction = cmp_shifted_instruction();

    // The AddWithCarry should be inside the first WriteFlag's BitTest
    let Stmt::WriteFlag { value, .. } = &instruction.semantics[0] else {
        panic!("expected flag write");
    };

    let Expr::BitTest { value: nzcv, bit: 3 } = value else {
        panic!("expected BitTest for N flag");
    };

    let Expr::TupleElement { value: add_with_carry, index: 1, .. } = nzcv.as_ref() else {
        panic!("expected TupleElement for NZCV");
    };

    let Expr::AddWithCarry { right, carry, width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
    assert!(matches!(carry.as_ref(), Expr::Bool(true)));

    let Expr::Unary { op, value: unary_value, .. } = right.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, UnaryOp::Not);

    let Expr::Shift { kind, .. } = unary_value.as_ref() else {
        panic!("expected Shift inside Not");
    };

    assert_eq!(*kind, ShiftKind::Lsl);
}
