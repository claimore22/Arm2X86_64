/// FCVTZS (scalar, integer): Floating-point convert to signed integer, rounding toward zero (scalar)
/// Stanford source: fcvtzs_scalar_int.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtzs_scalar_integer_instruction() -> Instruction {
    let datasize = BitWidth::W64;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTZS (scalar, integer): Convert floating-point to signed integer
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTZS,
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
            mnemonic: "FCVTZS (scalar, integer)".to_string(),
            encoding: Some("Floating-point convert to signed integer, rounding toward zero (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtzs_scalar_int.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtzs_scalar_integer_validates() {
    let instruction = fcvtzs_scalar_integer_instruction();
    validate_instruction(&instruction).expect("FCVTZS (scalar, integer) should validate");
}

#[test]
fn fcvtzs_scalar_integer_uses_vector_map() {
    let instruction = fcvtzs_scalar_integer_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTZS (scalar, integer)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTZS);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}