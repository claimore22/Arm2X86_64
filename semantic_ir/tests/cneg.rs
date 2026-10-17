/// CNEG (Conditional Negate) is an alias of CSNEG.
/// Stanford source: arm64_md_clean/cneg_csneg.md
/// It returns negated source if condition true, else source.
/// Maps to CSNEG with Rm=Rn and inverted condition: CSNEG Rd, Rn, Rn, !cond
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, UnaryOp, ValueType, Width,
    validate_instruction,
    primitives::Condition,
};

fn cneg_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // CNEG uses inverted condition from CSNEG
    let condition_holds = Expr::ConditionHolds {
        condition: Condition::Ne, // inverted from Eq
    };

    let xn = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // CSNEG with Rm=Rn: NOT(Xn) + 1
    let not_xn = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(xn.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let neg_xn = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(not_xn),
        right: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // result = if ConditionHolds(inverted_condition) then Xn else NOT(Xn) + 1
    // = if !cond then Xn else -Xn = if cond then -Xn else Xn
    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(xn),
        when_false: Box::new(neg_xn),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CNEG".to_string(),
            encoding: Some("Conditional negate (alias of CSNEG)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cneg_csneg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cneg_validates() {
    let instruction = cneg_instruction();

    validate_instruction(&instruction).expect("CNEG should validate");
}

#[test]
fn cneg_writes_result() {
    let instruction = cneg_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn cneg_uses_select_with_inverted_condition_and_negation() {
    let instruction = cneg_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Select { condition, when_true, when_false, ty } = value else {
        panic!("expected Select");
    };

    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Condition should be ConditionHolds with inverted condition (Ne instead of Eq)
    let Expr::ConditionHolds { condition } = condition.as_ref() else {
        panic!("expected ConditionHolds");
    };

    assert_eq!(*condition, Condition::Ne);

    // when_true = Xn (source register)
    assert!(matches!(when_true.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // when_false = NOT(Xn) + 1 (negated source)
    let Expr::Binary { op: op1, left, right, .. } = when_false.as_ref() else {
        panic!("expected Binary Add for NOT(Xn) + 1");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Add);
    assert!(matches!(right.as_ref(), Expr::Const { value: 1, .. }));

    // left = NOT(Xn)
    let Expr::Unary { op, value: unary_value, .. } = left.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, UnaryOp::Not);
    assert!(matches!(unary_value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}