/// CSET (Conditional Set) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cset_csinc.md
///
/// CSET is an alias of CSINC: `CSET Wd, cond` == `CSINC Wd, WZR, WZR, inv(cond)`.
/// The destination is set to 1 if the condition is TRUE, and 0 otherwise.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    primitives::Condition,
    validate_instruction,
};

fn cset_instruction() -> Instruction {
    // result = if ConditionHolds(cond) then 1 else 0
    let condition_holds = Expr::ConditionHolds {
        condition: Condition::Ne,
    };

    let one = Expr::Const {
        value: 1,
        ty: BitWidth::W64,
    };

    let zero = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };

    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(one),
        when_false: Box::new(zero),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CSET".to_string(),
            encoding: Some("Conditional set (alias of CSINC)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cset_csinc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cset_validates() {
    let instruction = cset_instruction();
    validate_instruction(&instruction).expect("CSET should validate");
}

#[test]
fn cset_writes_destination() {
    let instruction = cset_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn cset_selects_between_one_and_zero() {
    let instruction = cset_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Select {
        condition,
        when_true,
        when_false,
..
    } = value
    else {
        panic!("expected Select");
    };

    assert!(matches!(condition.as_ref(), Expr::ConditionHolds { .. }));
    assert!(matches!(when_true.as_ref(), Expr::Const { value: 1, .. }));
    assert!(matches!(when_false.as_ref(), Expr::Const { value: 0, .. }));
}
