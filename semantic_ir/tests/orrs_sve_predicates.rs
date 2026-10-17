/// ORRS - Bitwise inclusive OR predicates, setting the condition flags
/// Stanford source: arm64_md_clean/orrs_p_p_pp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn orrs_sve_predicates_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadPredReg { reg: 1, pl };
    let operand2 = Expr::ReadPredReg { reg: 2, pl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Orrs,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ORRS".to_string(),
            encoding: Some("Bitwise inclusive OR predicates, setting the condition flags".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "orrs_p_p_pp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn orrs_sve_predicates_validates() {
    let instruction = orrs_sve_predicates_instruction();
    validate_instruction(&instruction).expect("ORRS SVE predicates should validate");
}