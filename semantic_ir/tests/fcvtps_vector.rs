/// FCVTPS (vector): Floating-point convert to signed integer, rounding toward plus infinity (vector)
/// Stanford source: fcvtps_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtps_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTPS (vector): Convert to signed integer, rounding toward plus infinity
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTPS,
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
            mnemonic: "FCVTPS (vector)".to_string(),
            encoding: Some("Floating-point convert to signed integer, rounding toward plus infinity (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtps_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtps_vector_validates() {
    let instruction = fcvtps_vector_instruction();
    validate_instruction(&instruction).expect("FCVTPS (vector) should validate");
}

#[test]
fn fcvtps_vector_uses_vector_map() {
    let instruction = fcvtps_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTPS (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTPS);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}