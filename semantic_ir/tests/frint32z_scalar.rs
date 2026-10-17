/// FRINT32Z (scalar): Floating-point round to 32-bit integer toward zero (scalar)
/// Stanford source: frint32z_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frint32z_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINT32Z (scalar): Round to 32-bit integer toward zero
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRint32Z,
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
            mnemonic: "FRINT32Z (scalar)".to_string(),
            encoding: Some("Floating-point round to 32-bit integer toward zero (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frint32z_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frint32z_scalar_validates() {
    let instruction = frint32z_scalar_instruction();
    validate_instruction(&instruction).expect("FRINT32Z (scalar) should validate");
}

#[test]
fn frint32z_scalar_uses_vector_map() {
    let instruction = frint32z_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINT32Z (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRint32Z);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}