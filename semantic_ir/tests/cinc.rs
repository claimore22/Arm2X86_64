/// CINC (Conditional Increment) semantics are representable in the IR as an alias of CSINC.
/// Stanford source: arm64_md_clean/cinc_csinc.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
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

fn cinc_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // CINC uses inverted condition compared to CSINC
    let original_condition = Condition::Eq;
    let condition_holds = Expr::ConditionHolds {
        condition: invert_condition(original_condition),
    };

    // For CINC, Rn = Rm = source register (X2)
    let xn = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // result = if ConditionHolds(inverted_condition) then Xn + 1 else Xn
    let xn_plus_1 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(xn.clone()),
        right: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Select {
        condition: Box::new(condition_holds),
        when_true: Box::new(xn_plus_1),
        when_false: Box::new(xn),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CINC".to_string(),
            encoding: Some("Conditional increment (alias of CSINC)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cinc_csinc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cinc_validates() {
    let instruction = cinc_instruction();

    validate_instruction(&instruction).expect("CINC should validate");
}

#[test]
fn cinc_writes_result() {
    let instruction = cinc_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn cinc_uses_select_with_increment_on_true() {
    let instruction = cinc_instruction();

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

    // when_true = Xn + 1
    let Expr::Binary { op, left, right, .. } = when_true.as_ref() else {
        panic!("expected Binary Add for Xn + 1");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 1, .. }));

    // when_false = Xn
    assert!(matches!(when_false.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}