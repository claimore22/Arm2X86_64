/// FRINTA (scalar): Floating-point round to integral, to nearest with ties to away (scalar)
/// Stanford source: frinta_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frinta_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINTA (scalar): Round to integral, to nearest with ties to away
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRintA,
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
            mnemonic: "FRINTA (scalar)".to_string(),
            encoding: Some("Floating-point round to integral, to nearest with ties to away (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frinta_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frinta_scalar_validates() {
    let instruction = frinta_scalar_instruction();
    validate_instruction(&instruction).expect("FRINTA (scalar) should validate");
}

#[test]
fn frinta_scalar_uses_vector_map() {
    let instruction = frinta_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINTA (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRintA);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}