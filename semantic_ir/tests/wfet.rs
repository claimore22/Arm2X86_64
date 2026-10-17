/// WFET (Wait for event with timeout)
/// Stanford source: arm64_md_clean/wfet.md
/// Hint/barrier operation (placeholder Let).
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn wfet_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "wfet".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "wfet".to_string(), encoding: Some("WFET (Wait for event with timeout)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_WFxT".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "wfet.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn wfet_validates() {
    validate_instruction(&wfet_instruction()).expect("should validate");
}
