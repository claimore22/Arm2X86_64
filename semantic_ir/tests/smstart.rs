/// SMSTART (alias of MSR immediate): enable Streaming SVE/SME state semantics are representable in the IR.
/// Stanford source: arm64_md_clean/smstart_msr_imm.md
///
/// PSTATE.SM/ZA update; opaque.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn smstart_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "smstart".to_string(),
        value: Expr::Const { value: 1, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "SMSTART".to_string(), encoding: Some("Enable streaming SVE mode (MSR imm alias)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "smstart_msr_imm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn smstart_validates() {
    validate_instruction(&smstart_instruction()).expect("should validate");
}
