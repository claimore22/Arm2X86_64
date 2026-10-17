/// LSLV (Logical Shift Left Variable) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/lslv.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn lslv_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand1: Xn
    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // operand2: Xm (shift amount)
    let shift_amount = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // LSLV: ShiftReg(n, LSL, UInt(operand2) MOD datasize)
    // The shift amount is the lower bits of Xm modulo datasize
    // For 64-bit: shift = Xm[5:0] (mod 64)
    // We use Extract to get the lower 6 bits
    let shift_width = BitWidth::new(6).unwrap(); // log2(64) = 6 bits for shift amount
    let shift_mod = Expr::Extract {
        value: Box::new(shift_amount),
        lsb: 0,
        width: shift_width,
    };

    let result = Expr::Shift {
        kind: ShiftKind::Lsl,
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
            mnemonic: "LSLV".to_string(),
            encoding: Some("Logical shift left variable".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lslv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lslv_validates() {
    let instruction = lslv_instruction();

    validate_instruction(&instruction).expect("LSLV should validate");
}

#[test]
fn lslv_writes_result() {
    let instruction = lslv_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn lslv_uses_shift_with_modulo() {
    let instruction = lslv_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Shift { kind, value, amount, ty } = value else {
        panic!("expected Shift");
    };

    assert_eq!(*kind, ShiftKind::Lsl);
    assert_eq!(*ty, BitWidth::W64);

    // Value should be Xn
    assert!(matches!(value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // Amount should be Extract (modulo datasize)
    let Expr::Extract { value: ext_value, lsb, width } = amount.as_ref() else {
        panic!("expected Extract for modulo shift amount");
    };

    assert_eq!(*lsb, 0);
    let shift_width = BitWidth::new(6).unwrap();
    assert_eq!(*width, shift_width);

    // Extract from shift amount register
    assert!(matches!(ext_value.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
}