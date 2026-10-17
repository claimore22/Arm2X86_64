/// FCVTPU (vector): Floating-point convert to unsigned integer, rounding toward plus infinity (vector)
/// Stanford source: fcvtpu_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtpu_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTPU (vector): Convert to unsigned integer, rounding toward plus infinity
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTPU,
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
            mnemonic: "FCVTPU (vector)".to_string(),
            encoding: Some("Floating-point convert to unsigned integer, rounding toward plus infinity (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtpu_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtpu_vector_validates() {
    let instruction = fcvtpu_vector_instruction();
    validate_instruction(&instruction).expect("FCVTPU (vector) should validate");
}

#[test]
fn fcvtpu_vector_uses_vector_map() {
    let instruction = fcvtpu_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTPU (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTPU);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}