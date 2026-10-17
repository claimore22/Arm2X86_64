/// TRN1 - trn1_advsimd.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn trn1_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::AddP,
        operand: Box::new(operand),
        operand_datasize: BitWidth::W128,
        result_datasize: BitWidth::W128,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "TRN1".to_string(),
            encoding: Some("TRN1".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "trn1_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn trn1_validates() {
    let instruction = trn1_instruction();
    validate_instruction(&instruction).expect("TRN1 should validate");
}
