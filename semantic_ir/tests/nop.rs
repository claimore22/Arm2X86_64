/// NOP: no operation (HINT SystemHintOp_NOP).
/// Stanford source: arm64_md_clean/nop.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn nop_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "nop".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "NOP".to_string(), encoding: Some("No operation".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "nop.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn nop_validates() {
    validate_instruction(&nop_instruction()).expect("should validate");
}
