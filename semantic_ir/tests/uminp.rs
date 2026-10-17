/// UMINP: Unsigned minimum pairwise
/// Stanford source: uminp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uminp_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // UMINP: Unsigned minimum pairwise
    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::MinP,
        operand: Box::new(operand),
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
            mnemonic: "UMINP".to_string(),
            encoding: Some("Unsigned minimum pairwise".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uminp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uminp_validates() {
    let instruction = uminp_instruction();
    validate_instruction(&instruction).expect("UMINP should validate");
}

#[test]
fn uminp_uses_vector_pairwise() {
    let instruction = uminp_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand_datasize, result_datasize, esize, .. } = value else {
        panic!("expected VectorPairwise for UMINP");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::MinP);
    assert_eq!(*operand_datasize, BitWidth::W128);
    assert_eq!(*result_datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}