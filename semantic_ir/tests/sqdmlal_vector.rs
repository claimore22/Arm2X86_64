/// SQDMLAL, SQDMLAL2 (vector): Signed saturating doubling multiply-add long
/// Stanford source: sqdmlal_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sqdmlal_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // SQDMLAL/SQDMLAL2 (vector): Signed saturating doubling multiply-add long
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    // Create a clone of mul for the second use
    let mul2 = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul,
        left: Box::new(Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }),
        right: Box::new(Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let dmul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Add,
        left: Box::new(mul),
        right: Box::new(mul2),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Add,
        left: Box::new(addend),
        right: Box::new(dmul),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQDMLAL, SQDMLAL2 (vector)".to_string(),
            encoding: Some("Signed saturating doubling multiply-add long".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqdmlal_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqdmlal_vector_validates() {
    let instruction = sqdmlal_vector_instruction();
    validate_instruction(&instruction).expect("SQDMLAL, SQDMLAL2 (vector) should validate");
}

#[test]
fn sqdmlal_vector_uses_dmul_add() {
    let instruction = sqdmlal_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SQDMLAL (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Add);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Check right is DMul
    let Expr::VectorMap2 { op: dmul_op, left: dmul_left, right: dmul_right, .. } = right.as_ref() else {
        panic!("expected VectorMap2 for double multiply");
    };
    assert_eq!(*dmul_op, semantic_ir::primitives::VectorBinOp::Add);

    // Check dmul_left is Mul
    let Expr::VectorMap2 { op: mul_op, left: mul_left, right: mul_right, .. } = dmul_left.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::Mul);
    assert!(matches!(mul_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(mul_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}