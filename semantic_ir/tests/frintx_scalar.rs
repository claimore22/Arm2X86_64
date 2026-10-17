/// FRINTX (scalar): Floating-point round to integral exact, using current rounding mode (scalar)
/// Stanford source: frintx_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frintx_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINTX (scalar): Round to integral exact using current rounding mode
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRintX,
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
            mnemonic: "FRINTX (scalar)".to_string(),
            encoding: Some("Floating-point round to integral exact, using current rounding mode (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frintx_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frintx_scalar_validates() {
    let instruction = frintx_scalar_instruction();
    validate_instruction(&instruction).expect("FRINTX (scalar) should validate");
}

#[test]
fn frintx_scalar_uses_vector_map() {
    let instruction = frintx_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINTX (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRintX);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}