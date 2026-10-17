/// ASR (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/asr_reg.md
///
/// ASR (register) is an alias of ASRV with shift_type = ASR (from op2 field).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn asr_reg_instruction() -> Instruction {
    // This test instantiates the instruction at datasize = 64.
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let shift_amount = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // ASR (register) maps to ASRV with ShiftKind::Asr
    // The shift amount is the bottom 6 bits of the shift register
    let shift_width = BitWidth::new(6).unwrap();
    let shift_mod = Expr::Extract {
        value: Box::new(shift_amount),
        lsb: 0,
        width: shift_width,
    };

    let result = Expr::Shift {
        kind: ShiftKind::Asr,
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
            mnemonic: "ASR".to_string(),
            encoding: Some("Arithmetic shift right register (alias of ASRV)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asr_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asr_reg_validates() {
    let instruction = asr_reg_instruction();

    validate_instruction(&instruction).expect("ASR register should validate");
}

#[test]
fn asr_reg_writes_result() {
    let instruction = asr_reg_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn asr_reg_uses_asr_shift_with_modulo() {
    let instruction = asr_reg_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Shift { kind, value, amount, ty } = value else {
        panic!("expected Shift");
    };

    assert_eq!(*kind, ShiftKind::Asr);
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