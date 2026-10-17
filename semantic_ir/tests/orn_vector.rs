/// ORN (vector): Bitwise inclusive OR NOT (vector)
/// Stanford source: orn_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn orn_vector_instruction() -> Instruction {
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

    // ORN: Bitwise inclusive OR NOT
    let not_right = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Not,
        operand: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Orr,
        left: Box::new(left),
        right: Box::new(not_right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ORN".to_string(),
            encoding: Some("Bitwise inclusive OR NOT (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "orn_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn orn_vector_validates() {
    let instruction = orn_vector_instruction();
    validate_instruction(&instruction).expect("ORN should validate");
}

#[test]
fn orn_vector_uses_or_not() {
    let instruction = orn_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for ORN");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Orr);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);

    // Check right is NOT
    let Expr::VectorMap { op: not_op, operand, .. } = right.as_ref() else {
        panic!("expected VectorMap for NOT");
    };
    assert_eq!(*not_op, semantic_ir::primitives::VectorUnaryOp::Not);
}