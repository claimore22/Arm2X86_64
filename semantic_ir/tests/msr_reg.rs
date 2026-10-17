/// MSR (register): move general-purpose register to System register.
/// Stanford source: arm64_md_clean/msr_reg.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn msr_reg_instruction() -> Instruction {
    let semantics = vec![Stmt::WriteSpecial {
        reg: SpecialReg::GcrEl1,
        value: Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 },
        width: Width::Fixed(BitWidth::W64),
    }];

    Instruction {
        id: InstructionId { mnemonic: "MSR".to_string(), encoding: Some("Move GPR to System register".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "msr_reg.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn msr_reg_validates() {
    validate_instruction(&msr_reg_instruction()).expect("should validate");
}
