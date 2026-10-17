/// SMIN (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/smin_reg.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn smin_reg_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // SMIN: result = Min(SInt(operand1), SInt(operand2))
    let condition = Expr::Binary {
        op: semantic_ir::BinOp::LtSigned,
        left: Box::new(operand1.clone()),
        right: Box::new(operand2.clone()),
        ty: ValueType::Bool,
    };

    let result = Expr::Select {
        condition: Box::new(condition),
        when_true: Box::new(operand1),
        when_false: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SMIN".to_string(),
            encoding: Some("Signed minimum (register)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "smin_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn smin_reg_validates() {
    let instruction = smin_reg_instruction();

    validate_instruction(&instruction).expect("SMIN (register) should validate");
}

#[test]
fn smin_reg_uses_conditional_select() {
    let instruction = smin_reg_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Select { condition, when_true, when_false, ty } = value else {
        panic!("expected Select");
    };

    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Condition should be operand1 < operand2 (signed)
    let Expr::Binary { op, left, right, .. } = condition.as_ref() else {
        panic!("expected Binary LtSigned");
    };

    assert_eq!(*op, semantic_ir::BinOp::LtSigned);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));

    // when_true = operand1 (if operand1 < operand2), when_false = operand2
    assert!(matches!(when_true.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(when_false.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
}