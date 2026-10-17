/// MADD (Multiply-Add) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/madd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn madd_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand1 = Xn
    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // operand2 = Xm
    let operand2 = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // operand3 = Xa
    let operand3 = Expr::ReadReg {
        reg: Reg::X(4),
        width: BitWidth::W64,
    };

    // result = operand3 + (operand1 * operand2)
    let mul = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(operand1),
        right: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(operand3),
        right: Box::new(mul),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MADD".to_string(),
            encoding: Some("Multiply-add".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "madd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn madd_validates() {
    let instruction = madd_instruction();

    validate_instruction(&instruction).expect("MADD should validate");
}

#[test]
fn madd_uses_mul_and_add() {
    let instruction = madd_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // result = operand3 + (operand1 * operand2)
    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Add");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(4), .. }));

    // right = operand1 * operand2
    let Expr::Binary { op: op2, left: left2, right: right2, .. } = right.as_ref() else {
        panic!("expected Binary Mul");
    };

    assert_eq!(*op2, semantic_ir::BinOp::Mul);
    assert!(matches!(left2.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right2.as_ref(), Expr::ReadReg { reg: Reg::X(3), .. }));
}