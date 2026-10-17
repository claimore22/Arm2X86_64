/// TSB (Trace synchronization barrier)
/// Stanford source: arm64_md_clean/tsb.md
/// Hint/barrier operation (placeholder Let).
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn tsb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "tsb".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "tsb".to_string(), encoding: Some("TSB (Trace synchronization barrier)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_TRF".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "tsb.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn tsb_validates() {
    validate_instruction(&tsb_instruction()).expect("should validate");
}
