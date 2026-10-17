/// FCMLA (by element) - fcmla_z_p_zzz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn fcmla_by_element_instruction() -> Instruction {
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
            mnemonic: "FCMLA (by element)".to_string(),
            encoding: Some("FCMLA (by element)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmla_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcmla_by_element_validates() {
    let instruction = fcmla_by_element_instruction();
    validate_instruction(&instruction).expect("FCMLA (by element) should validate");
}
