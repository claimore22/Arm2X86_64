/// MRS: move System register to general-purpose register (opaque system register read).
/// Stanford source: arm64_md_clean/mrs.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn mrs_instruction() -> Instruction {
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: Expr::ReadSpecial { reg: SpecialReg::GcrEl1, width: BitWidth::W64 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "MRS".to_string(), encoding: Some("Move System register to GPR".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "mrs.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn mrs_validates() {
    validate_instruction(&mrs_instruction()).expect("should validate");
}
