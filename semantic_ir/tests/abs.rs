/// ABS (Absolute Value) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/abs.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn abs_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // ABS: result = Abs(SInt(operand1))
    // Abs(x) = if x < 0 then -x else x
    let is_negative = Expr::Binary {
        op: semantic_ir::BinOp::LtSigned,
        left: Box::new(operand1.clone()),
        right: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bool,
    };

    let neg_operand1 = Expr::Unary {
        op: semantic_ir::UnaryOp::Neg,
        value: Box::new(operand1.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Select {
        condition: Box::new(is_negative),
        when_true: Box::new(neg_operand1),
        when_false: Box::new(operand1),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ABS".to_string(),
            encoding: Some("Absolute value".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "abs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn abs_validates() {
    let instruction = abs_instruction();

    validate_instruction(&instruction).expect("ABS should validate");
}

#[test]
fn abs_uses_select_with_negation() {
    let instruction = abs_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Select { condition, when_true, when_false, .. } = value else {
        panic!("expected Select for absolute value");
    };

    // Condition: operand1 < 0
    let Expr::Binary { op, left, right, .. } = condition.as_ref() else {
        panic!("expected Binary LtSigned for condition");
    };

    assert_eq!(*op, semantic_ir::BinOp::LtSigned);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 0, .. }));

    // when_true = -operand1
    let Expr::Unary { op, value: unary_value, .. } = when_true.as_ref() else {
        panic!("expected Unary Neg for when_true");
    };

    assert_eq!(*op, semantic_ir::UnaryOp::Neg);
    assert!(matches!(unary_value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // when_false = operand1
    assert!(matches!(when_false.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}