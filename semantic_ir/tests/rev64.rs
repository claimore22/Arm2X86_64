/// REV64: Reverse elements in 64-bit doublewords (vector)
/// Stanford source: rev64.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn rev64_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // REV64: Reverse elements in 64-bit doublewords
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Rev64,
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
            mnemonic: "REV64".to_string(),
            encoding: Some("Reverse elements in 64-bit doublewords (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rev64.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rev64_validates() {
    let instruction = rev64_instruction();
    validate_instruction(&instruction).expect("REV64 should validate");
}

#[test]
fn rev64_uses_vector_map() {
    let instruction = rev64_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for REV64");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Rev64);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}