/// CBZ (Compare and Branch on Zero) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cbz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn cbz_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand1 = Xt
    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // offset = SignExtend(imm19::'00')
    let offset = Expr::Const {
        value: 0x1000, // Example branch offset
        ty: BitWidth::W64,
    };

    // CBZ: if IsZero(operand1) then BranchTo(PC + offset) else BranchNotTaken
    // In IR: BranchConditional with condition = IsZero(operand1)
    let condition = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
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
            mnemonic: "CBZ".to_string(),
            encoding: Some("Compare and branch on zero".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cbz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cbz_validates() {
    let instruction = cbz_instruction();

    validate_instruction(&instruction).expect("CBZ should validate");
}

#[test]
fn cbz_uses_branch_conditional() {
    let instruction = cbz_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &instruction.semantics[0] else {
        panic!("expected BranchConditional");
    };

    // Condition should be operand1 == 0
    let Expr::Binary { op, left, right, .. } = condition else {
        panic!("expected Binary Eq");
    };

    assert_eq!(*op, semantic_ir::BinOp::Eq);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 0, .. }));

    // Target should be PC + offset
    let Expr::Binary { op: op2, left: left2, right: right2, .. } = target else {
        panic!("expected Binary Add for target");
    };

    assert_eq!(*op2, semantic_ir::BinOp::Add);
    assert!(matches!(left2.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, .. }));
    assert!(matches!(right2.as_ref(), Expr::Const { .. }));
}