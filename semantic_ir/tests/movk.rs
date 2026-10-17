/// MOVK (Move Wide with Keep) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/movk.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn movk_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // result = X(d) (read current value)
    let current = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    // pos = hw << 4 (0, 16, 32, or 48)
    let pos = 32;

    // Place 16-bit immediate at position pos
    let imm16 = 0xABCD;
    let shifted_imm = Expr::Const {
        value: (imm16 as u128) << pos,
        ty: BitWidth::W64,
    };

    // Create mask for the 16-bit field at position pos
    let mask = Expr::Const {
        value: 0xFFFFu128 << pos,
        ty: BitWidth::W64,
    };

    // result = (current & ~mask) | shifted_imm
    let not_mask = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(mask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let current_and_not_mask = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(current),
        right: Box::new(not_mask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(current_and_not_mask),
        right: Box::new(shifted_imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVK".to_string(),
            encoding: Some("Move wide with keep".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movk.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn movk_validates() {
    let instruction = movk_instruction();

    validate_instruction(&instruction).expect("MOVK should validate");
}

#[test]
fn movk_writes_result() {
    let instruction = movk_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn movk_uses_current_value_and_merges_immediate() {
    let instruction = movk_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    // result = (current & ~mask) | shifted_imm
    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Or for final result");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Or);

    // right = shifted_imm
    assert!(matches!(right.as_ref(), Expr::Const { .. }));

    // left = current & ~mask
    let Expr::Binary { op: op2, left: curr_part, right: not_mask_part, .. } = left.as_ref() else {
        panic!("expected Binary And for current part");
    };

    assert_eq!(*op2, semantic_ir::BinOp::And);

    // curr_part = current register value
    assert!(matches!(curr_part.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));

    // not_mask_part = NOT(mask)
    let Expr::Unary { op: not_op, .. } = not_mask_part.as_ref() else {
        panic!("expected Unary Not for mask");
    };
    assert_eq!(*not_op, semantic_ir::UnaryOp::Not);
}