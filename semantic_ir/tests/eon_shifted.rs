/// EON (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/eon.md
///
/// Xd = Xn XOR NOT(shift(Xm)). Requires no special features.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, UnaryOp,
    ValueType, Width, validate_instruction,
};

fn eon_shifted_instruction() -> Instruction {
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

    // EON: operand1 XOR NOT(operand2)
    let not_operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Xor,
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
            mnemonic: "EON".to_string(),
            encoding: Some("Bitwise exclusive-OR NOT (shifted register)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eon.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eon_shifted_validates() {
    let instruction = eon_shifted_instruction();
    validate_instruction(&instruction).expect("EON should validate");
}

#[test]
fn eon_shifted_reads_two_registers() {
    let instruction = eon_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, left, right, .. } = value else {
        panic!("expected Binary");
    };

    assert_eq!(*op, semantic_ir::BinOp::Xor);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(
        right.as_ref(),
        Expr::Unary {
            op: UnaryOp::Not,
..
        }
    ));
}

#[test]
fn eon_shifted_writes_destination() {
    let instruction = eon_shifted_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}
