/// EXTR (Extract Register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/extr.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn extr_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand1 = Xn
    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // operand2 = Xm
    let operand2 = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // lsb = imm6 (0-63)
    let lsb = 16; // Example

    // EXTR: result = concat(operand1, operand2)[(lsb+datasize)-1:lsb]
    // This is equivalent to: (operand1 << lsb) | (operand2 >> (64 - lsb))
    // For 64-bit: result = (operand1 << lsb) | (operand2 >> (64 - lsb))

    let shift_left = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(operand1.clone()),
        amount: Box::new(Expr::Const {
            value: lsb,
            ty: BitWidth::new(6).unwrap(),
        }),
        ty: BitWidth::W64,
    };

    let shift_right = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsr,
        value: Box::new(operand2),
        amount: Box::new(Expr::Const {
            value: 64 - lsb,
            ty: BitWidth::new(6).unwrap(),
        }),
        ty: BitWidth::W64,
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(shift_left),
        right: Box::new(shift_right),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "EXTR".to_string(),
            encoding: Some("Extract register".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "extr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn extr_validates() {
    let instruction = extr_instruction();

    validate_instruction(&instruction).expect("EXTR should validate");
}

#[test]
fn extr_uses_shift_and_or() {
    let instruction = extr_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Or");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Or);

    // left = operand1 << lsb
    let Expr::Shift { kind: kind1, value: val1, amount: amt1, .. } = left.as_ref() else {
        panic!("expected Shift LSL");
    };

    assert_eq!(*kind1, semantic_ir::ShiftKind::Lsl);
    assert!(matches!(val1.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(amt1.as_ref(), Expr::Const { value: 16, .. }));

    // right = operand2 >> (64 - lsb)
    let Expr::Shift { kind: kind2, value: val2, amount: amt2, .. } = right.as_ref() else {
        panic!("expected Shift LSR");
    };

    assert_eq!(*kind2, semantic_ir::ShiftKind::Lsr);
    assert!(matches!(val2.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
    assert!(matches!(amt2.as_ref(), Expr::Const { value: 48, .. }));
}