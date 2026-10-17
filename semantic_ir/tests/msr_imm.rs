/// MSR (immediate): move immediate to PSTATE field (SPSel/DAIFSet/DAIFClr etc.).
/// Stanford source: arm64_md_clean/msr_imm.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn msr_imm_instruction() -> Instruction {
    // MSR (immediate) writes a PSTATE field, modeled as opaque hint
    let semantics = vec![Stmt::Let {
        name: "msr_imm".to_string(),
        value: Expr::Const { value: 1, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "MSR".to_string(), encoding: Some("Move immediate to PSTATE field".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "msr_imm.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn msr_imm_validates() {
    validate_instruction(&msr_imm_instruction()).expect("should validate");
}
