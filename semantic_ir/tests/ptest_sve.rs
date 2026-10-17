/// PTEST - Set condition flags for predicate
/// Stanford source: arm64_md_clean/ptest_p_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ptest_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadPredReg { reg: 1, pl };

    let result = Expr::PredicateReduce {
        op: semantic_ir::primitives::SVEPredicateOp::Ptest,
        operand: Box::new(operand),
        pl,
    };

    let semantics = vec![Stmt::Discard { value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "PTEST".to_string(),
            encoding: Some("Set condition flags for predicate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ptest_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ptest_sve_validates() {
    let instruction = ptest_sve_instruction();
    validate_instruction(&instruction).expect("PTEST SVE should validate");
}