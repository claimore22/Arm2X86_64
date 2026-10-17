/// CSEL (Conditional Select) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/csel.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
    primitives::Condition,
};

fn csel_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // condition = cond (4-bit condition code)
    // For testing, we use a specific condition (e.g., EQ = 0b0000)
    let condition = Expr::Condition(Condition::Eq);

    // result = if ConditionHolds(condition) then Xn else Xm
    let xn = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let xm = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    let condition_holds = Expr::ConditionHolds {
        condition: Condition::Eq,
    };

    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(xn),
        when_false: Box::new(xm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CSEL".to_string(),
            encoding: Some("Conditional select".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "csel.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn csel_validates() {
    let instruction = csel_instruction();

    validate_instruction(&instruction).expect("CSEL should validate");
}

#[test]
fn csel_writes_result() {
    let instruction = csel_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn csel_uses_select_with_condition_holds() {
    let instruction = csel_instruction();

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

    // when_false = Xm
    assert!(matches!(when_false.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
}