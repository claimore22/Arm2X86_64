/// MLS (vector): Multiply-subtract from accumulator (vector)
/// Stanford source: mls_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn mls_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

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

    // MLS (vector): Multiply-subtract
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub,
        left: Box::new(addend),
        right: Box::new(mul),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MLS (vector)".to_string(),
            encoding: Some("Multiply-subtract from accumulator (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mls_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mls_vector_validates() {
    let instruction = mls_vector_instruction();
    validate_instruction(&instruction).expect("MLS (vector) should validate");
}

#[test]
fn mls_vector_uses_mul_sub() {
    let instruction = mls_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for MLS (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Sub);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Check right is Mul
    let Expr::VectorMap2 { op: mul_op, left: mul_left, right: mul_right, .. } = right.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::Mul);
    assert!(matches!(mul_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(mul_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}