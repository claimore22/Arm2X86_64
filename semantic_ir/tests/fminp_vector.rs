/// FMINP (vector): Floating-point minimum pairwise (vector)
/// Stanford source: fminp_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fminp_vector_instruction() -> Instruction {
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

    // FMINP (vector): Pairwise minimum
    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::FMinP,
        operand: Box::new(left),
        operand_datasize: BitWidth::W128,
        result_datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINP (vector)".to_string(),
            encoding: Some("Floating-point minimum pairwise (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminp_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fminp_vector_validates() {
    let instruction = fminp_vector_instruction();
    validate_instruction(&instruction).expect("FMINP (vector) should validate");
}

#[test]
fn fminp_vector_uses_vector_pairwise() {
    let instruction = fminp_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand_datasize, result_datasize, esize, .. } = value else {
        panic!("expected VectorPairwise for FMINP (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::FMinP);
    assert_eq!(*operand_datasize, BitWidth::W128);
    assert_eq!(*result_datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}