/// FRINTN (scalar): Floating-point round to integral, to nearest with ties to even (scalar)
/// Stanford source: frintn_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frintn_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINTN (scalar): Round to integral, to nearest with ties to even
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRintN,
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
            mnemonic: "FRINTN (scalar)".to_string(),
            encoding: Some("Floating-point round to integral, to nearest with ties to even (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frintn_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frintn_scalar_validates() {
    let instruction = frintn_scalar_instruction();
    validate_instruction(&instruction).expect("FRINTN (scalar) should validate");
}

#[test]
fn frintn_scalar_uses_vector_map() {
    let instruction = frintn_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINTN (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRintN);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}