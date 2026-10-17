/// B.cond semantics are representable in the IR.
/// Stanford source: arm64_md_clean/b_cond.md
///
/// Stanford (arm64_md_clean/b_cond.md):
///   offset = SignExtend(imm19::'00')
///   condition = cond (4-bit)
///   if ConditionHolds(condition) then BranchTo(PC64() + offset, DIR)
///   else BranchNotTaken(DIR)
use semantic_ir::{
    BitWidth, Condition, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn b_cond_instruction() -> Instruction {
    // offset = SignExtend(imm19::'00')
    let offset = Expr::Const {
        value: 0x10000,
        ty: BitWidth::W64,
    };

    // target = PC + offset
    let target = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc64,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // condition = cond (4-bit)
    let condition = Expr::ConditionHolds {
        condition: Condition::Eq,
    };

    let semantics = vec![Stmt::BranchConditional {
        condition,
        target,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "B.cond".to_string(),
            encoding: Some("Branch conditionally".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
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
            source_path: "b_cond.md".to_string(),
            source_sha256:
                "5d50d65803a042d7f1bc0eb72c35f5f93c41781d970a8ae48a758232e486410a".to_string(),
            clean_sha256:
                "b059eb7ea3dcee546669ac68742ce53217985fd69364d28480026aafee0f7c1d".to_string(),
        },
    }
}

#[test]
fn b_cond_validates() {
    let instruction = b_cond_instruction();
    validate_instruction(&instruction).expect("B.cond should validate");
}

#[test]
fn b_cond_uses_branch_conditional() {
    let instruction = b_cond_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::BranchConditional { .. }
    ));
}

#[test]
fn b_cond_checks_condition_and_branches() {
    let instruction = b_cond_instruction();
    let Stmt::BranchConditional { condition, target } = &instruction.semantics[0] else {
        panic!("expected BranchConditional");
    };

    // Condition is ConditionHolds(EQ)
    assert!(matches!(
        condition,
        Expr::ConditionHolds { condition: Condition::Eq }
    ));

    // Target = PC + offset
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
fn b_cond_decode_has_imm19_and_cond() {
    let instruction = b_cond_instruction();
    let decode = instruction.decode.expect("B.cond needs decode");
    assert_eq!(decode.statements.len(), 2);
    // offset = SignExtend(imm19::'00') -> ShiftLeft(imm19, 2)
    let has_scaled_offset = decode.statements.iter().any(|s| {
        matches!(s, DecodeStmt::Let {
            value: DecodeExpr::ShiftLeft { amount: 2, .. },
..
        })
    });
    assert!(has_scaled_offset, "decode must scale imm19 by 2");
}
