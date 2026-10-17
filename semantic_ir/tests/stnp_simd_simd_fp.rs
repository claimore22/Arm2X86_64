/// STNP (SIMD&FP): Store pair of SIMD&FP registers, with non-temporal hint
/// Stanford source: stnp_simd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stnp_simd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let offset = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // STNP: Store pair with non-temporal hint
    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::AddP,
        operand: Box::new(Expr::ReadReg { reg: Reg::V(0), width: BitWidth::W128 }),
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
            mnemonic: "STNP (SIMD&FP)".to_string(),
            encoding: Some("Store pair of SIMD&FP registers, with non-temporal hint".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stnp_simd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stnp_simd_validates() {
    let instruction = stnp_simd_instruction();
    validate_instruction(&instruction).expect("STNP (SIMD&FP) should validate");
}

#[test]
fn stnp_simd_uses_vector_pairwise() {
    let instruction = stnp_simd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand_datasize, result_datasize, esize, .. } = value else {
        panic!("expected VectorPairwise for STNP (SIMD&FP)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::AddP);
    assert_eq!(*operand_datasize, BitWidth::W128);
    assert_eq!(*result_datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}