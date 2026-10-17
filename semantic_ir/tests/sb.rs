/// SB (Speculation barrier)
/// Stanford source: arm64_md_clean/sb.md
/// SpeculationBarrier; NOP without FEAT_SPECRES.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn sb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "sb".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "sb".to_string(), encoding: Some("SB (Speculation barrier)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPECRES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sb.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn sb_validates() {
    validate_instruction(&sb_instruction()).expect("should validate");
}
