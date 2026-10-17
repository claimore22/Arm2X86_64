/// FRINT64X (vector): Floating-point round to 64-bit integer, using current rounding mode (vector)
/// Stanford source: frint64x_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frint64x_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINT64X (vector): Round to 64-bit integer using current rounding mode
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRint64X,
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
            mnemonic: "FRINT64X (vector)".to_string(),
            encoding: Some("Floating-point round to 64-bit integer, using current rounding mode (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frint64x_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frint64x_vector_validates() {
    let instruction = frint64x_vector_instruction();
    validate_instruction(&instruction).expect("FRINT64X (vector) should validate");
}

#[test]
fn frint64x_vector_uses_vector_map() {
    let instruction = frint64x_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINT64X (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRint64X);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}