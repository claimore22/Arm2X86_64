/// SVC (Supervisor call)
/// Stanford source: arm64_md_clean/svc.md
/// AArch64_CallSupervisor: opaque exception to EL1.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn svc_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "svc".to_string(),
        value: Expr::Const { value: 0, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "svc".to_string(), encoding: Some("SVC (Supervisor call)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "svc.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn svc_validates() {
    validate_instruction(&svc_instruction()).expect("should validate");
}
