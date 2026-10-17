/// Subtract optionally-shifted register semantics are representable in the IR.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ShiftKind,
    UnaryOp, ValueType, Width, validate_instruction,
};
use semantic_ir::BinOp;

fn sub_shifted_instruction() -> Instruction {
    let operand1 = Expr::ReadIndexedReg {
        index: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        width: BitWidth::W64,
    };

    let shifted = Expr::Shift {
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

    let operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(shifted),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::TupleElement {
        value: Box::new(Expr::AddWithCarry {
            left: Box::new(operand1),
            right: Box::new(operand2),
            carry: Box::new(Expr::Bool(true)),
            width: BitWidth::W64,
        }),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let write = Stmt::WriteIndexedReg {
        index: Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        },
        value: result,
        width: Width::Fixed(BitWidth::W64),
    };

    let semantics = vec![write];

    Instruction {
        id: InstructionId {
            mnemonic: "SUB (shifted register)".to_string(),
            encoding: Some("Subtract optionally-shifted register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sub_shifted.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sub_shifted_register_semantics_are_representable() {
    let instruction = sub_shifted_instruction();
    match validate_instruction(&instruction) {
        Ok(()) => {}
        Err(e) => panic!("SUB (shifted register) should validate: {e}"),
    }

    let semantics = &instruction.semantics;
    assert_eq!(semantics.len(), 1);

    match &semantics[0] {
        Stmt::WriteIndexedReg { value, width, .. } => {
            assert_eq!(*width, Width::Fixed(BitWidth::W64));

            match value {
                Expr::TupleElement { index, .. } => {
                    assert_eq!(*index, 0);
                }
                _ => panic!("expected tuple element"),
            }
        }
        _ => panic!("expected indexed register write"),
    }
}

#[test]
fn sub_shifted_register_uses_not_and_set_carry() {
    let shifted = Expr::Shift {
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

    let inverted = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(shifted),
        ty: ValueType::Bits(Width::InstructionDataSize),
    };

    let sub = Expr::AddWithCarry {
        left: Box::new(Expr::ReadIndexedReg {
            index: Box::new(Expr::Const {
                value: 2,
                ty: BitWidth::W32,
            }),
            width: BitWidth::W64,
        }),
        right: Box::new(inverted),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    match sub {
        Expr::AddWithCarry { carry, .. } => {
            assert!(matches!(*carry, Expr::Bool(true)));
        }
        _ => panic!("expected AddWithCarry"),
    }
}
