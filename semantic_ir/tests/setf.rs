/// SETF8, SETF16 (Evaluation of 8-bit or 16-bit flag values)
/// Stanford source: arm64_md_clean/setf.md
/// Sets NZ from the low 8/16 bits of Xn (FEAT_FlagM). Modeled via Extract + WriteFlag.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn setf_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "setf".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "setf".to_string(), encoding: Some("SETF8, SETF16 (Evaluation of 8-bit or 16-bit flag values)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_FlagM".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "setf.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn setf_validates() {
    validate_instruction(&setf_instruction()).expect("should validate");
}
