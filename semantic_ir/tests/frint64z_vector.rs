/// FRINT64Z (vector): Floating-point round to 64-bit integer toward zero (vector)
/// Stanford source: frint64z_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frint64z_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRINT64Z (vector): Round to 64-bit integer toward zero
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRint64Z,
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
            mnemonic: "FRINT64Z (vector)".to_string(),
            encoding: Some("Floating-point round to 64-bit integer toward zero (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frint64z_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frint64z_vector_validates() {
    let instruction = frint64z_vector_instruction();
    validate_instruction(&instruction).expect("FRINT64Z (vector) should validate");
}

#[test]
fn frint64z_vector_uses_vector_map() {
    let instruction = frint64z_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRINT64Z (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FRint64Z);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}