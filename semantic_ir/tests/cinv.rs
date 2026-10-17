/// CINV (Conditional Invert) semantics are representable in the IR as an alias of CSINV.
/// Stanford source: arm64_md_clean/cinv_csinv.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, UnaryOp, ValueType, Width,
    validate_instruction,
    primitives::Condition,
};

fn invert_condition(cond: Condition) -> Condition {
    match cond {
        Condition::Eq => Condition::Ne,
        Condition::Ne => Condition::Eq,
        Condition::Cs => Condition::Cc,
        Condition::Cc => Condition::Cs,
        Condition::Mi => Condition::Pl,
        Condition::Pl => Condition::Mi,
        Condition::Vs => Condition::Vc,
        Condition::Vc => Condition::Vs,
        Condition::Hi => Condition::Ls,
        Condition::Ls => Condition::Hi,
        Condition::Ge => Condition::Lt,
        Condition::Lt => Condition::Ge,
        Condition::Gt => Condition::Le,
        Condition::Le => Condition::Gt,
        Condition::Al => Condition::Nv,
        Condition::Nv => Condition::Al,
    }
}

fn cinv_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // CINV uses inverted condition compared to CSINV
    let original_condition = Condition::Eq;
    let condition_holds = Expr::ConditionHolds {
        condition: invert_condition(original_condition),
    };

    // For CINV, Rn = Rm = source register (X2)
    let xn = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // result = if ConditionHolds(inverted_condition) then Xn else NOT(Xn)
    // This is CSINV with inverted condition and Rn=Rm=X2:
    // CSINV: if !cond then Rn else NOT(Rn) = if cond then NOT(Rn) else Rn
    let not_xn = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(xn.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(not_xn),
        when_false: Box::new(xn),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CINV".to_string(),
            encoding: Some("Conditional invert (alias of CSINV)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cinv_csinv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cinv_validates() {
    let instruction = cinv_instruction();

    validate_instruction(&instruction).expect("CINV should validate");
}

#[test]
fn cinv_writes_result() {
    let instruction = cinv_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn cinv_uses_select_with_not_on_true() {
    let instruction = cinv_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Select { condition, when_true, when_false, ty } = value else {
        panic!("expected Select");
    };

    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Condition should be ConditionHolds with inverted condition (Ne for Eq)
    let Expr::ConditionHolds { condition } = condition.as_ref() else {
        panic!("expected ConditionHolds");
    };

    assert_eq!(*condition, Condition::Ne);

    // when_true = NOT(Xn)
    let Expr::Unary { op, value: unary_value, .. } = when_true.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, UnaryOp::Not);
    assert!(matches!(unary_value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // when_false = Xn
    assert!(matches!(when_false.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}