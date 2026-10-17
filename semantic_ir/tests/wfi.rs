/// WFI (Wait for interrupt)
/// Stanford source: arm64_md_clean/wfi.md
/// Hint/barrier operation (placeholder Let).
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn wfi_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "wfi".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "wfi".to_string(), encoding: Some("WFI (Wait for interrupt)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "wfi.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn wfi_validates() {
    validate_instruction(&wfi_instruction()).expect("should validate");
}
