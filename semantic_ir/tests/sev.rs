/// SEV (Send event)
/// Stanford source: arm64_md_clean/sev.md
/// SendEvent() hint.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn sev_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "sev".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "sev".to_string(), encoding: Some("SEV (Send event)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sev.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn sev_validates() {
    validate_instruction(&sev_instruction()).expect("should validate");
}
