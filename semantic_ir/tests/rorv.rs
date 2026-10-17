/// RORV (Rotate Right Variable) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rorv.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn rorv_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let shift_amount = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    let shift_width = BitWidth::new(6).unwrap();
    let shift_mod = Expr::Extract {
        value: Box::new(shift_amount),
        lsb: 0,
        width: shift_width,
    };

    let result = Expr::Shift {
        kind: ShiftKind::Ror,
        value: Box::new(operand1),
        amount: Box::new(shift_mod),
        ty: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "RORV".to_string(),
            encoding: Some("Rotate right variable".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rorv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rorv_validates() {
    let instruction = rorv_instruction();

    validate_instruction(&instruction).expect("RORV should validate");
}

#[test]
fn rorv_writes_result() {
    let instruction = rorv_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn rorv_uses_ror_shift_with_modulo() {
    let instruction = rorv_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Shift { kind, value, amount, ty } = value else {
        panic!("expected Shift");
    };

    assert_eq!(*kind, ShiftKind::Ror);
    assert_eq!(*ty, BitWidth::W64);
    assert!(matches!(value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    let Expr::Extract { value: ext_value, lsb, width } = amount.as_ref() else {
        panic!("expected Extract for modulo shift amount");
    };

    assert_eq!(*lsb, 0);
    let shift_width = BitWidth::new(6).unwrap();
    assert_eq!(*width, shift_width);
    assert!(matches!(ext_value.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
}