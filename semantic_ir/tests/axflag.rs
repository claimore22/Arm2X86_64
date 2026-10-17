/// AXFLAG semantics are representable in the IR.
/// Stanford source: arm64_md_clean/axflag.md
/// Stanford (arm64_md_no_nav_clean/axflag.md):
///   z = PSTATE.Z OR PSTATE.V
///   c = PSTATE.C AND NOT(PSTATE.V)
///   PSTATE.N = 0
///   PSTATE.Z = z
///   PSTATE.C = c
///   PSTATE.V = 0
///
/// Requires FEAT_FlagM2.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Flag, Instruction, InstructionId,
    Provenance, Stmt, ValueType, Width, validate_instruction,
};

fn axflag_instruction() -> Instruction {
    // z = PSTATE.Z OR PSTATE.V
    let z = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(Expr::ReadFlag { flag: Flag::Z }),
        right: Box::new(Expr::ReadFlag { flag: Flag::V }),
        ty: ValueType::Bool,
    };

    // c = PSTATE.C AND NOT(PSTATE.V)
    let not_v = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(Expr::ReadFlag { flag: Flag::V }),
        ty: ValueType::Bool,
    };

    let c = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(Expr::ReadFlag { flag: Flag::C }),
        right: Box::new(not_v),
        ty: ValueType::Bool,
    };

    let semantics = vec![
        Stmt::WriteFlag {
            flag: Flag::N,
            value: Expr::Bool(false),
        },
        Stmt::WriteFlag {
            flag: Flag::Z,
            value: z,
        },
        Stmt::WriteFlag {
            flag: Flag::C,
            value: c,
        },
        Stmt::WriteFlag {
            flag: Flag::V,
            value: Expr::Bool(false),
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "AXFLAG".to_string(),
            encoding: Some("Convert floating-point condition flags".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FlagM2".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![],
        }),
        semantics,
        provenance: Provenance {
            source_path: "axflag.md".to_string(),
            source_sha256:
                "ac47736ba16ce0229b56ffa75e5a8e2fa3657f362463aa662d4fbbbf8b8a6bbc".to_string(),
            clean_sha256:
                "4bf69cf9c8197ebf8571261d7737040d50032eb24ddc5b88ac9dac56a1f707e6".to_string(),
        },
    }
}

#[test]
fn axflag_validates() {
    let instruction = axflag_instruction();
    validate_instruction(&instruction).expect("AXFLAG should validate");
}

#[test]
fn axflag_requires_flagm2_feature() {
    let instruction = axflag_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_FlagM2".to_string()]);
}

#[test]
fn axflag_writes_all_four_flags() {
    let instruction = axflag_instruction();
    assert_eq!(instruction.semantics.len(), 4);
    let flags: Vec<Flag> = instruction.semantics.iter().map(|s| {
        match s {
            Stmt::WriteFlag { flag, .. } => *flag,
            _ => panic!("expected WriteFlag"),
        }
    }).collect();
    assert_eq!(flags, vec![Flag::N, Flag::Z, Flag::C, Flag::V]);
}

#[test]
fn axflag_computes_z_c_correctly() {
    let instruction = axflag_instruction();
    let Stmt::WriteFlag { value: z_val, .. } = &instruction.semantics[1] else {
        panic!("expected WriteFlag for Z");
    };
    // z = Z OR V
    let Expr::Binary { op, left, right, ty: ValueType::Bool } = z_val else {
        panic!("expected Binary Or for z");
    };
    assert_eq!(*op, semantic_ir::BinOp::Or);
    assert!(matches!(left.as_ref(), Expr::ReadFlag { flag: Flag::Z }));
    assert!(matches!(right.as_ref(), Expr::ReadFlag { flag: Flag::V }));

    let Stmt::WriteFlag { value: c_val, .. } = &instruction.semantics[2] else {
        panic!("expected WriteFlag for C");
    };
    // c = C AND NOT(V)
    let Expr::Binary { op, left, right, ty: ValueType::Bool } = c_val else {
        panic!("expected Binary And for c");
    };
    assert_eq!(*op, semantic_ir::BinOp::And);
    assert!(matches!(left.as_ref(), Expr::ReadFlag { flag: Flag::C }));
    let Expr::Unary { op: unary_op, value, ty: ValueType::Bool } = right.as_ref() else {
        panic!("expected Unary Not for NOT(V)");
    };
    assert_eq!(*unary_op, semantic_ir::UnaryOp::Not);
    assert!(matches!(value.as_ref(), Expr::ReadFlag { flag: Flag::V }));

    // N = 0, V = 0
    let Stmt::WriteFlag { value: n_val, .. } = &instruction.semantics[0] else {
        panic!("expected WriteFlag for N");
    };
    assert!(matches!(n_val, Expr::Bool(false)));

    let Stmt::WriteFlag { value: v_val, .. } = &instruction.semantics[3] else {
        panic!("expected WriteFlag for V");
    };
    assert!(matches!(v_val, Expr::Bool(false)));
}