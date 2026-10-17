/// AND (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/and_log_shift.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn and_shifted_instruction() -> Instruction {
    // This test instantiates the instruction at datasize = 64.
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::Shift {
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
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(operand1),
        right: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AND".to_string(),
            encoding: Some("Bitwise AND shifted register".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "and_log_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn and_shifted_register_validates() {
    let instruction = and_shifted_instruction();

    validate_instruction(&instruction).expect("AND shifted register should validate");
}

#[test]
fn and_shifted_register_writes_result_only() {
    let instruction = and_shifted_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn and_shifted_register_uses_binary_and() {
    let instruction = and_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Binary { op, left, right, ty } = value else {
        panic!("expected Binary And");
    };

    assert_eq!(*op, semantic_ir::BinOp::And);
    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Verify left operand is a register read
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // Verify right operand is a shifted register
    assert!(matches!(
        right.as_ref(),
        Expr::Shift {
            kind: ShiftKind::Lsl,
..
        }
    ));
}

#[test]
fn and_shifted_register_shift_details() {
    let instruction = and_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Binary { right, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::Shift {
        kind,
        value: shift_value,
        amount,
        ty,
    } = right.as_ref()
    else {
        panic!("expected Shift");
    };

    assert_eq!(*kind, ShiftKind::Lsl);
    assert_eq!(*ty, BitWidth::W64);

    // Shift amount is 4
    assert!(matches!(amount.as_ref(), Expr::Const { value: 4, ty: BitWidth::W32 }));

    // Shift input is X3
    assert!(matches!(
        shift_value.as_ref(),
        Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }
    ));
}
