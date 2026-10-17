/// CSETM (Conditional Set Mask) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/csetm_csinv.md
///
/// CSETM is an alias of CSINV: `CSETM Wd, cond` == `CSINV Wd, WZR, WZR, inv(cond)`.
/// All bits of the destination are set to 1 if the condition is TRUE, else 0.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    primitives::Condition,
    validate_instruction,
};

fn csetm_instruction() -> Instruction {
    // result = if ConditionHolds(cond) then all_ones else 0
    let condition_holds = Expr::ConditionHolds {
        condition: Condition::Ne,
    };

    let all_ones = Expr::Const {
        value: u64::MAX as u128,
        ty: BitWidth::W64,
    };

    let zero = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };

    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(all_ones),
        when_false: Box::new(zero),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CSETM".to_string(),
            encoding: Some("Conditional set mask (alias of CSINV)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "csetm_csinv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn csetm_validates() {
    let instruction = csetm_instruction();
    validate_instruction(&instruction).expect("CSETM should validate");
}

#[test]
fn csetm_writes_destination() {
    let instruction = csetm_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn csetm_selects_between_all_ones_and_zero() {
    let instruction = csetm_instruction();

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
    assert!(matches!(
        when_true.as_ref(),
        Expr::Const { value, .. } if *value == u64::MAX as u128
    ));
    assert!(matches!(when_false.as_ref(), Expr::Const { value: 0, .. }));
}
