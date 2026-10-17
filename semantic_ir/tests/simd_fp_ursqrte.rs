/// URSQRTE: URSQRTE
/// Stanford source: simd_fp_ursqrte.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_ursqrte_instruction() -> Instruction {
    let datasize = BitWidth::W32;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W32,
    };

    // URSQRTE: URSQRTE
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FPSqrt,
        operand: Box::new(operand),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "URSQRTE".to_string(),
            encoding: Some("URSQRTE".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_ursqrte.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_ursqrte_validates() {
    let instruction = simd_fp_ursqrte_instruction();
    validate_instruction(&instruction).expect("URSQRTE should validate");
}

#[test]
fn simd_fp_ursqrte_uses_vector_map() {
    let instruction = simd_fp_ursqrte_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for URSQRTE");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FPSqrt);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
}
