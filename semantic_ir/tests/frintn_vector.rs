/// FRINTN (vector): Floating-point round to integral, to nearest with ties to even (vector)
/// Stanford source: frintn_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frintn_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINTN (vector): Round to integral, to nearest with ties to even
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRintN,
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
            mnemonic: "FRINTN (vector)".to_string(),
            encoding: Some("Floating-point round to integral, to nearest with ties to even (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frintn_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frintn_vector_validates() {
    let instruction = frintn_vector_instruction();
    validate_instruction(&instruction).expect("FRINTN (vector) should validate");
}

#[test]
fn frintn_vector_uses_vector_map() {
    let instruction = frintn_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINTN (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRintN);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}