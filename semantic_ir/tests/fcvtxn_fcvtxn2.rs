/// FCVTXN, FCVTXN2: Floating-point convert to lower precision narrow, rounding to odd (vector)
/// Stanford source: fcvtxn_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtxn_fcvtxn2_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTXN/FCVTXN2: Convert to lower precision, rounding to odd
    // Uses VectorMap with narrowing convert op
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for FCVTXN
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
            mnemonic: "FCVTXN, FCVTXN2".to_string(),
            encoding: Some("Floating-point convert to lower precision narrow, rounding to odd (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtxn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtxn_fcvtxn2_validates() {
    let instruction = fcvtxn_fcvtxn2_instruction();
    validate_instruction(&instruction).expect("FCVTXN, FCVTXN2 should validate");
}

#[test]
fn fcvtxn_fcvtxn2_uses_vector_map() {
    let instruction = fcvtxn_fcvtxn2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTXN, FCVTXN2");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}