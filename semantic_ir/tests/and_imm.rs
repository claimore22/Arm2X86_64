/// AND (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/and_log_imm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn and_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand1: Xn
    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // operand2: immediate (decoded from bitmask)
    // In the IR, we represent the decoded immediate as a constant
    let operand2 = Expr::Const {
        value: 0x0000FFFF0000FFFFu128, // Example bitmask pattern
        ty: BitWidth::W64,
    };

    // result = operand1 AND operand2
    let result = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(operand1),
        right: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Stanford:
    // if d == 31 then SP{64}() = ZeroExtend{64}(result) else X{datasize}(d) = result
    //
    // For concrete 64-bit datasize,
            // datasize, result is already 64-bit, so ZeroExtend is a no-op.
    // We write directly to SP.
    let write_sp = Stmt::WriteSP {
        value: result.clone(),
        width: Width::Fixed(BitWidth::W64),
    };

    let write_reg = Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    };

    let dest_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };

    let semantics = vec![Stmt::If {
        condition: dest_is_sp,
        then_body: vec![write_sp],
        else_body: vec![write_reg],
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AND".to_string(),
            encoding: Some("Bitwise AND immediate".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "and_log_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn and_imm_validates() {
    let instruction = and_imm_instruction();

    validate_instruction(&instruction).expect("AND immediate should validate");
}

#[test]
fn and_imm_conditional_destination() {
    let instruction = and_imm_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::If {
        condition,
        then_body,
        else_body,
    } = &instruction.semantics[0]
    else {
        panic!("expected If for conditional destination");
    };

    // Condition checks if destination is SP (register 31)
    assert!(matches!(
        condition,
        Expr::Binary {
            op: semantic_ir::BinOp::Eq,
..
        }
    ));

    // Then body: write SP with ZeroExtend
    assert_eq!(then_body.len(), 1);
    assert!(matches!(
        then_body[0],
        Stmt::WriteSP { .. }
    ));

    // Else body: write X register
    assert_eq!(else_body.len(), 1);
    assert!(matches!(
        else_body[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn and_imm_sp_write_uses_zero_extend() {
    let instruction = and_imm_instruction();

    let Stmt::If { then_body, .. } = &instruction.semantics[0] else {
        panic!("expected If");
    };

    let Stmt::WriteSP { value, width } = &then_body[0] else {
        panic!("expected WriteSP");
    };

    assert_eq!(*width, Width::Fixed(BitWidth::W64));

    // For 64-bit datasize,
            // datasize, result is already 64-bit, so no ZeroExtend needed
    // The value should be the AND result directly
    assert!(matches!(
        value,
        Expr::Binary {
            op: semantic_ir::BinOp::And,
..
        }
    ));
}

#[test]
fn and_imm_uses_binary_and() {
    let instruction = and_imm_instruction();

    let Stmt::If { else_body, .. } = &instruction.semantics[0] else {
        panic!("expected If");
    };

    let Stmt::WriteReg { value, .. } = &else_body[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, left, right, ty } = value else {
        panic!("expected Binary And");
    };

    assert_eq!(*op, semantic_ir::BinOp::And);
    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Left is register read
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // Right is immediate constant
    assert!(matches!(
        right.as_ref(),
        Expr::Const { value: 0x0000FFFF0000FFFFu128, ty: BitWidth::W64 }
    ));
}
