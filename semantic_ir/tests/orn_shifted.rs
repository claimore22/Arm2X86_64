/// ORN (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/orn_log_shift.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, UnaryOp,
    ValueType, Width, validate_instruction,
};

fn orn_shifted_instruction() -> Instruction {
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

    // ORN: operand1 OR NOT(operand2)
    let not_operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(operand1),
        right: Box::new(not_operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ORN".to_string(),
            encoding: Some("Bitwise OR NOT shifted register".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "orn_log_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn orn_shifted_register_validates() {
    let instruction = orn_shifted_instruction();

    validate_instruction(&instruction).expect("ORN shifted register should validate");
}

#[test]
fn orn_shifted_register_writes_result_only() {
    let instruction = orn_shifted_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn orn_shifted_register_uses_or_not() {
    let instruction = orn_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Binary { op, left, right, ty } = value else {
        panic!("expected Binary Or");
    };

    assert_eq!(*op, semantic_ir::BinOp::Or);
    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // Right operand should be NOT(shifted register)
    let Expr::Unary { op: unary_op, value: unary_value, ty: unary_ty } = right.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*unary_op, UnaryOp::Not);
    assert_eq!(*unary_ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    assert!(matches!(
        unary_value.as_ref(),
        Expr::Shift {
            kind: ShiftKind::Lsl,
..
        }
    ));
}
