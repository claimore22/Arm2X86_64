/// CFINV (Invert Carry Flag) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cfinv.md
/// Stanford (arm64_md_no_nav_clean/cfinv.md):
///   PSTATE.C = NOT(PSTATE.C)
///
/// Requires FEAT_FlagM.
use semantic_ir::{
    Expr, Flag, Instruction, InstructionId, Provenance, Stmt, ValueType, validate_instruction,
};

fn cfinv_instruction() -> Instruction {
    // CFINV: PSTATE.C = NOT(PSTATE.C)
    let condition = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(Expr::ReadFlag { flag: Flag::C }),
        ty: ValueType::Bool,
    };

    let semantics = vec![Stmt::WriteFlag {
        flag: Flag::C,
        value: condition,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CFINV".to_string(),
            encoding: Some("Invert carry flag".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FlagM".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cfinv.md".to_string(),
            source_sha256: "eee1bd16d38849a00db2a91ea218efe81403c5afdc70032eb59ae0901766be1a".to_string(),
            clean_sha256: "6a4b02a2b86a0c2c7933bb6eeb5e55801cdeaaec04a5cf6f98a7f2168be000db".to_string(),
        },
    }
}

#[test]
fn cfinv_validates() {
    let instruction = cfinv_instruction();

    validate_instruction(&instruction).expect("CFINV should validate");
}

#[test]
fn cfinv_requires_flagm_feature() {
    let instruction = cfinv_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_FlagM".to_string()]);
}

#[test]
fn cfinv_inverts_carry_flag() {
    let instruction = cfinv_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteFlag { flag, value } = &instruction.semantics[0] else {
        panic!("expected WriteFlag");
    };

    assert_eq!(*flag, Flag::C);

    let Expr::Unary { op, value, ty: ValueType::Bool } = value else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, semantic_ir::UnaryOp::Not);
    assert!(matches!(value.as_ref(), Expr::ReadFlag { flag: Flag::C }));
}