/// BC.cond semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bc_cond.md
/// Stanford (arm64_md_no_nav_clean/bc_cond.md):
///   Same as B.cond but requires FEAT_HBC.
///   offset = SignExtend(imm19::'00')
///   condition = cond (4-bit)
///   if ConditionHolds(condition) then BranchTo(PC64() + offset, DIR)
///   else BranchNotTaken(DIR)
use semantic_ir::{
    BitWidth, Condition, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn bc_cond_instruction() -> Instruction {
    let offset = Expr::Const {
        value: 0x10000,
        ty: BitWidth::W64,
    };

    let target = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc64,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let condition = Expr::ConditionHolds {
        condition: Condition::Eq,
    };

    let semantics = vec![Stmt::BranchConditional {
        condition,
        target,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BC.cond".to_string(),
            encoding: Some("Branch consistent conditionally".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_HBC".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "offset".to_string(),
                    value: DecodeExpr::ShiftLeft {
                        value: Box::new(DecodeExpr::UInt("imm19".to_string())),
                        amount: 2,
                    },
                },
                DecodeStmt::Let {
                    name: "condition".to_string(),
                    value: DecodeExpr::UInt("cond".to_string()),
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "bc_cond.md".to_string(),
            source_sha256:
                "45dd667176b33f8cfe16efea387b5af7e05cbfc4f156565d4ef8e1ca76d3b741".to_string(),
            clean_sha256:
                "569fb8582fa39ba5d537e8a153987b17f560be282ed02848b9deab0857ec6ab6".to_string(),
        },
    }
}

#[test]
fn bc_cond_validates() {
    let instruction = bc_cond_instruction();
    validate_instruction(&instruction).expect("BC.cond should validate");
}

#[test]
fn bc_cond_requires_hbc_feature() {
    let instruction = bc_cond_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_HBC".to_string()]);
}

#[test]
fn bc_cond_uses_branch_conditional() {
    let instruction = bc_cond_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::BranchConditional { .. }
    ));
}

#[test]
fn bc_cond_checks_condition_and_branches() {
    let instruction = bc_cond_instruction();
    let Stmt::BranchConditional { condition, target } = &instruction.semantics[0] else {
        panic!("expected BranchConditional");
    };

    assert!(matches!(
        condition,
        Expr::ConditionHolds { condition: Condition::Eq }
    ));

    let Expr::Binary { op, left, right, ty: ValueType::Bits(Width::Fixed(BitWidth::W64)) } = target else {
        panic!("expected Binary Add for PC-relative target");
    };
    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(
        left.as_ref(),
        Expr::ReadSpecial { reg: SpecialReg::Pc64, .. }
    ));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));
}

#[test]
fn bc_cond_decode_has_imm19_and_cond() {
    let instruction = bc_cond_instruction();
    let decode = instruction.decode.expect("BC.cond needs decode");
    assert_eq!(decode.statements.len(), 2);
    let has_scaled_offset = decode.statements.iter().any(|s| {
        matches!(s, DecodeStmt::Let {
            value: DecodeExpr::ShiftLeft { amount: 2, .. },
..
        })
    });
    assert!(has_scaled_offset, "decode must scale imm19 by 2");
}
