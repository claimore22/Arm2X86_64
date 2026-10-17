/// SMADDL (Signed Multiply-Add Long) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/smaddl.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn smaddl_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand1 = Xn (lower 32 bits)
    let operand1 = Expr::Extract {
        value: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        lsb: 0,
        width: BitWidth::W32,
    };

    // operand2 = Xm (lower 32 bits)
    let operand2 = Expr::Extract {
        value: Box::new(Expr::ReadReg {
            reg: Reg::X(3),
            width: BitWidth::W64,
        }),
        lsb: 0,
        width: BitWidth::W32,
    };

    // operand3 = Xa (64 bits)
    let operand3 = Expr::ReadReg {
        reg: Reg::X(4),
        width: BitWidth::W64,
    };

    // SMADDL: result = operand3 + SInt(operand1) * SInt(operand2)
    // operand1 and operand2 are 32-bit, sign-extended to 64-bit for multiplication
    let op1_sext = Expr::SignExtend {
        value: Box::new(operand1),
        to: BitWidth::W64,
    };

    let op2_sext = Expr::SignExtend {
        value: Box::new(operand2),
        to: BitWidth::W64,
    };

    let mul = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(op1_sext),
        right: Box::new(op2_sext),
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
            mnemonic: "SMADDL".to_string(),
            encoding: Some("Signed multiply-add long".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "smaddl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn smaddl_validates() {
    let instruction = smaddl_instruction();

    validate_instruction(&instruction).expect("SMADDL should validate");
}

#[test]
fn smaddl_uses_sign_extend_mul_and_add() {
    let instruction = smaddl_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Add");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(4), .. }));

    // right = SInt(operand1) * SInt(operand2)
    let Expr::Binary { op: op2, left: left2, right: right2, .. } = right.as_ref() else {
        panic!("expected Binary Mul");
    };

    assert_eq!(*op2, semantic_ir::BinOp::Mul);

    // Both operands should be SignExtend of Extract
    assert!(matches!(left2.as_ref(), Expr::SignExtend { .. }));
    assert!(matches!(right2.as_ref(), Expr::SignExtend { .. }));
}