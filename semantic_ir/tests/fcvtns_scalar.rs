/// FCVTNS (scalar): Floating-point convert to signed integer, rounding to nearest with ties to even (scalar)
/// Stanford source: fcvtns_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtns_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTNS (scalar): Convert to signed integer, rounding to nearest with ties to even
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTNS,
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
            mnemonic: "FCVTNS (scalar)".to_string(),
            encoding: Some("Floating-point convert to signed integer, rounding to nearest with ties to even (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtns_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtns_scalar_validates() {
    let instruction = fcvtns_scalar_instruction();
    validate_instruction(&instruction).expect("FCVTNS (scalar) should validate");
}

#[test]
fn fcvtns_scalar_uses_vector_map() {
    let instruction = fcvtns_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTNS (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTNS);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}