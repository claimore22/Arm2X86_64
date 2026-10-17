/// REV32 (vector): Reverse elements in 32-bit words (vector)
/// Stanford source: rev32_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn rev32_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // REV32 (vector): Reverse elements in 32-bit words
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Rev32,
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
            mnemonic: "REV32".to_string(),
            encoding: Some("Reverse elements in 32-bit words (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rev32_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rev32_vector_validates() {
    let instruction = rev32_vector_instruction();
    validate_instruction(&instruction).expect("REV32 (vector) should validate");
}

#[test]
fn rev32_vector_uses_vector_map() {
    let instruction = rev32_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for REV32 (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Rev32);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}