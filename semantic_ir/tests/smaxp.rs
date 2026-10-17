/// SMAXP: Signed maximum pairwise
/// Stanford source: smaxp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn smaxp_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // SMAXP: Signed maximum pairwise
    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::MaxP,
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
            mnemonic: "SMAXP".to_string(),
            encoding: Some("Signed maximum pairwise".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "smaxp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn smaxp_validates() {
    let instruction = smaxp_instruction();
    validate_instruction(&instruction).expect("SMAXP should validate");
}

#[test]
fn smaxp_uses_vector_pairwise() {
    let instruction = smaxp_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand_datasize, result_datasize, esize, .. } = value else {
        panic!("expected VectorPairwise for SMAXP");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::MaxP);
    assert_eq!(*operand_datasize, BitWidth::W128);
    assert_eq!(*result_datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}