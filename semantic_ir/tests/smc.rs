/// SMC (Secure monitor call)
/// Stanford source: arm64_md_clean/smc.md
/// AArch64_CallSecureMonitor: opaque exception to EL3.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn smc_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "smc".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "smc".to_string(), encoding: Some("SMC (Secure monitor call)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "smc.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn smc_validates() {
    validate_instruction(&smc_instruction()).expect("should validate");
}
