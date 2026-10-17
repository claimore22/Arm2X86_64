/// FRSQRTE: Floating-point reciprocal square root estimate
/// Stanford source: frsqrte.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frsqrte_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRSQRTE: Reciprocal square root estimate
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRecipSqrtEstimate,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRSQRTE".to_string(),
            encoding: Some("Floating-point reciprocal square root estimate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frsqrte.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frsqrte_validates() {
    let instruction = frsqrte_instruction();
    validate_instruction(&instruction).expect("FRSQRTE should validate");
}

#[test]
fn frsqrte_uses_vector_map() {
    let instruction = frsqrte_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRSQRTE");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRecipSqrtEstimate);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}