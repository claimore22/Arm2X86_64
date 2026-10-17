/// VSQRT (vector): Vector square root
/// Stanford source: vsqrt_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vsqrt_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    // VSQRT (vector): Vector square root
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for VSQRT
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
            mnemonic: "VSQRT (vector)".to_string(),
            encoding: Some("Vector square root".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vsqrt_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vsqrt_vector_validates() {
    let instruction = vsqrt_vector_instruction();
    validate_instruction(&instruction).expect("VSQRT (vector) should validate");
}

#[test]
fn vsqrt_vector_uses_vector_map() {
    let instruction = vsqrt_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VSQRT (vector)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}