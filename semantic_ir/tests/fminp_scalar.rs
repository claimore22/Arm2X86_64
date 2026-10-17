/// FMINP (scalar): Floating-point minimum of pair of elements (scalar)
/// Stanford source: fminp_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fminp_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // FMINP (scalar): Pairwise minimum
    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::FMinP,
        operand: Box::new(left),
        operand_datasize: BitWidth::W128,
        result_datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINP (scalar)".to_string(),
            encoding: Some("Floating-point minimum of pair of elements (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminp_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fminp_scalar_validates() {
    let instruction = fminp_scalar_instruction();
    validate_instruction(&instruction).expect("FMINP (scalar) should validate");
}

#[test]
fn fminp_scalar_uses_vector_pairwise() {
    let instruction = fminp_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand_datasize, result_datasize, esize, .. } = value else {
        panic!("expected VectorPairwise for FMINP (scalar)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::FMinP);
    assert_eq!(*operand_datasize, BitWidth::W128);
    assert_eq!(*result_datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}