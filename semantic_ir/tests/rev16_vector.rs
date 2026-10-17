/// REV16 (vector): Reverse elements in 16-bit halfwords (vector)
/// Stanford source: rev16_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn rev16_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // REV16 (vector): Reverse elements in 16-bit halfwords
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Rev16,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "REV16".to_string(),
            encoding: Some("Reverse elements in 16-bit halfwords (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rev16_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rev16_vector_validates() {
    let instruction = rev16_vector_instruction();
    validate_instruction(&instruction).expect("REV16 (vector) should validate");
}

#[test]
fn rev16_vector_uses_vector_map() {
    let instruction = rev16_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for REV16 (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Rev16);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}