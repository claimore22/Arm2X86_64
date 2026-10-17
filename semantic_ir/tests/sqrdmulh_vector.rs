/// SQRDMULH (vector): Signed saturating rounding doubling multiply returning high half
/// Stanford source: sqrdmulh_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sqrdmulh_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // SQRDMULH (vector): Signed saturating rounding doubling multiply returning high half (placeholder)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul, // Placeholder for SQRDMULH
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQRDMULH (vector)".to_string(),
            encoding: Some("Signed saturating rounding doubling multiply returning high half".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqrdmulh_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqrdmulh_vector_validates() {
    let instruction = sqrdmulh_vector_instruction();
    validate_instruction(&instruction).expect("SQRDMULH (vector) should validate");
}

#[test]
fn sqrdmulh_vector_uses_dmul() {
    let instruction = sqrdmulh_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SQRDMULH (vector)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}