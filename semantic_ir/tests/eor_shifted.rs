/// EOR (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/eor_log_shift.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn eor_shifted_instruction() -> Instruction {
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
        op: semantic_ir::BinOp::Xor,
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
            mnemonic: "EOR".to_string(),
            encoding: Some("Bitwise XOR shifted register".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eor_log_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eor_shifted_register_validates() {
    let instruction = eor_shifted_instruction();

    validate_instruction(&instruction).expect("EOR shifted register should validate");
}

#[test]
fn eor_shifted_register_writes_result_only() {
    let instruction = eor_shifted_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn eor_shifted_register_uses_binary_xor() {
    let instruction = eor_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Binary { op, left, right, ty } = value else {
        panic!("expected Binary Xor");
    };

    assert_eq!(*op, semantic_ir::BinOp::Xor);
    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(
        right.as_ref(),
        Expr::Shift {
            kind: ShiftKind::Lsl,
..
        }
    ));
}
