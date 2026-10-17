/// REV (predicate) - Reverse all elements in a predicate
/// Stanford source: arm64_md_clean/rev_p_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn rev_sve_predicate_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let operand = Expr::ReadPredReg { reg: 1, pl };

    let result = Expr::PredicateMap {
        op: semantic_ir::primitives::SVEPredicateOp::Rev,
        operand: Box::new(operand),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "REV".to_string(),
            encoding: Some("Reverse all elements in a predicate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rev_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rev_sve_predicate_validates() {
    let instruction = rev_sve_predicate_instruction();
    validate_instruction(&instruction).expect("REV SVE predicate should validate");
}