/// SQDMULL, SQDMULL2 (vector): Signed saturating doubling multiply long
/// Stanford source: sqdmull_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sqdmull_vector_instruction() -> Instruction {
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

    // SQDMULL/SQDMULL2 (vector): Signed saturating doubling multiply long
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    // Clone mul for second use
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

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: dmul,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQDMULL, SQDMULL2 (vector)".to_string(),
            encoding: Some("Signed saturating doubling multiply long".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqdmull_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqdmull_vector_validates() {
    let instruction = sqdmull_vector_instruction();
    validate_instruction(&instruction).expect("SQDMULL, SQDMULL2 (vector) should validate");
}

#[test]
fn sqdmull_vector_uses_dmul() {
    let instruction = sqdmull_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SQDMULL (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Add);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);

    // Check left is Mul
    let Expr::VectorMap2 { op: mul_op, left: mul_left, right: mul_right, .. } = left.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::Mul);
    assert!(matches!(mul_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(mul_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}