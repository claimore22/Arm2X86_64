/// EOR (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/eor_log_imm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn eor_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::Const {
        value: 0xAAAAAAAAAAAAAAAAu128,
        ty: BitWidth::W64,
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Xor,
        left: Box::new(operand1),
        right: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // For concrete 64-bit datasize,
            // datasize, result is already 64-bit, so ZeroExtend is a no-op.
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
            mnemonic: "EOR".to_string(),
            encoding: Some("Bitwise XOR immediate".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eor_log_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eor_imm_validates() {
    let instruction = eor_imm_instruction();

    validate_instruction(&instruction).expect("EOR immediate should validate");
}

#[test]
fn eor_imm_conditional_destination() {
    let instruction = eor_imm_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::If {
        condition,
        then_body,
        else_body,
    } = &instruction.semantics[0]
    else {
        panic!("expected If for conditional destination");
    };

    assert!(matches!(
        condition,
        Expr::Binary {
            op: semantic_ir::BinOp::Eq,
..
        }
    ));

    assert_eq!(then_body.len(), 1);
    assert!(matches!(then_body[0], Stmt::WriteSP { .. }));

    assert_eq!(else_body.len(), 1);
    assert!(matches!(else_body[0], Stmt::WriteReg { reg: Reg::X(1), .. }));
}

#[test]
fn eor_imm_uses_binary_xor() {
    let instruction = eor_imm_instruction();

    let Stmt::If { else_body, .. } = &instruction.semantics[0] else {
        panic!("expected If");
    };

    let Stmt::WriteReg { value, .. } = &else_body[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, left, right, ty } = value else {
        panic!("expected Binary Xor");
    };

    assert_eq!(*op, semantic_ir::BinOp::Xor);
    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(
        right.as_ref(),
        Expr::Const { value: 0xAAAAAAAAAAAAAAAAu128, ty: BitWidth::W64 }
    ));
}
