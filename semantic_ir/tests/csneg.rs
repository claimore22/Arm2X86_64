/// CSNEG (Conditional Select Negation) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/csneg.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, UnaryOp, ValueType, Width,
    validate_instruction,
    primitives::Condition,
};

fn csneg_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let condition_holds = Expr::ConditionHolds {
        condition: Condition::Eq,
    };

    let xn = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let xm = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // result = if ConditionHolds(condition) then Xn else NOT(Xm) + 1
    let not_xm = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(xm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let neg_xm = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(not_xm),
        right: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(xn),
        when_false: Box::new(neg_xm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CSNEG".to_string(),
            encoding: Some("Conditional select negation".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "csneg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn csneg_validates() {
    let instruction = csneg_instruction();

    validate_instruction(&instruction).expect("CSNEG should validate");
}

#[test]
fn csneg_writes_result() {
    let instruction = csneg_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn csneg_uses_select_with_negation() {
    let instruction = csneg_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Select { condition, when_true, when_false, ty } = value else {
        panic!("expected Select");
    };

    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Condition should be ConditionHolds
    let Expr::ConditionHolds { condition } = condition.as_ref() else {
        panic!("expected ConditionHolds");
    };

    assert_eq!(*condition, Condition::Eq);

    // when_true = Xn
    assert!(matches!(when_true.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // when_false = NOT(Xm) + 1
    let Expr::Binary { op: op1, left, right, .. } = when_false.as_ref() else {
        panic!("expected Binary Add for NOT(Xm) + 1");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Add);
    assert!(matches!(right.as_ref(), Expr::Const { value: 1, .. }));

    // left = NOT(Xm)
    let Expr::Unary { op, value: unary_value, .. } = left.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, UnaryOp::Not);
    assert!(matches!(unary_value.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
}