/// CBNZ (Compare and Branch on Nonzero) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cbnz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn cbnz_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let offset = Expr::Const {
        value: 0x1000,
        ty: BitWidth::W64,
    };

    // CBNZ: if !IsZero(operand1) then BranchTo(PC + offset) else BranchNotTaken
    let condition = Expr::Binary {
        op: semantic_ir::BinOp::Ne,
        left: Box::new(operand1),
        right: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bool,
    };

    let target = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: semantic_ir::SpecialReg::Pc,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::BranchConditional {
        condition,
        target,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CBNZ".to_string(),
            encoding: Some("Compare and branch on nonzero".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cbnz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cbnz_validates() {
    let instruction = cbnz_instruction();

    validate_instruction(&instruction).expect("CBNZ should validate");
}

#[test]
fn cbnz_uses_branch_conditional_with_ne() {
    let instruction = cbnz_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &instruction.semantics[0] else {
        panic!("expected BranchConditional");
    };

    let Expr::Binary { op, left, right, .. } = condition else {
        panic!("expected Binary Ne");
    };

    assert_eq!(*op, semantic_ir::BinOp::Ne);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 0, .. }));

    let Expr::Binary { op: op2, left: left2, right: right2, .. } = target else {
        panic!("expected Binary Add for target");
    };

    assert_eq!(*op2, semantic_ir::BinOp::Add);
    assert!(matches!(left2.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, .. }));
    assert!(matches!(right2.as_ref(), Expr::Const { .. }));
}