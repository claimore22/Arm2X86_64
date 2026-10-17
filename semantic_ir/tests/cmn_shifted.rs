/// CMN (shifted register) is an alias of ADDS (shifted register) that discards the result.
/// Stanford source: arm64_md_clean/cmn_adds_addsub_shift.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn cmn_shifted_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadIndexedReg {
        index: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        width: BitWidth::W64,
    };

    let operand2 = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(Expr::ReadIndexedReg {
            index: Box::new(Expr::Const {
                value: 3,
                ty: BitWidth::W32,
            }),
            width: BitWidth::W64,
        }),
        amount: Box::new(Expr::Const {
            value: 4,
            ty: BitWidth::W32,
        }),
        ty: BitWidth::W64,
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(false)),
        width: BitWidth::W64,
    };

    // CMN discards the result (tuple element 0), only extracts flags (tuple element 1)
    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

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
            mnemonic: "CMN".to_string(),
            encoding: Some("Compare negative (shifted register, alias of ADDS)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmn_adds_addsub_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmn_shifted_validates() {
    let instruction = cmn_shifted_instruction();

    validate_instruction(&instruction).expect("CMN shifted register should validate");
}

#[test]
fn cmn_shifted_writes_only_flags() {
    let instruction = cmn_shifted_instruction();

    // CMN writes only 4 flags, no result register
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
fn cmn_shifted_uses_zero_carry() {
    let instruction = cmn_shifted_instruction();

    // The flags are extracted from AddWithCarry result (tuple element 1)
    let Stmt::WriteFlag { value, .. } = &instruction.semantics[0] else {
        panic!("expected flag write");
    };

    let Expr::BitTest { value: nzcv, bit: _ } = value else {
        panic!("expected BitTest for flag");
    };

    let Expr::TupleElement {
        value: add_with_carry,
        index: 1,
..
    } = nzcv.as_ref()
    else {
        panic!("expected TupleElement index 1 for flags");
    };

    let Expr::AddWithCarry { right, carry, width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
    assert!(matches!(carry.as_ref(), Expr::Bool(false)));

    let Expr::Shift { kind, value, amount, ty: _ } = right.as_ref() else {
        panic!("expected Shift for shifted register");
    };

    assert_eq!(*kind, ShiftKind::Lsl);
    assert!(matches!(value.as_ref(), Expr::ReadIndexedReg { index: _, width: BitWidth::W64, .. }));
    assert!(matches!(amount.as_ref(), Expr::Const { value: 4, ty: BitWidth::W32, .. }));
}
