/// MRRS: move System register to two adjacent GPRs (opaque read, pair write).
/// Stanford source: arm64_md_clean/mrrs.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn mrrs_instruction() -> Instruction {
    let val = Expr::ReadSpecial { reg: SpecialReg::GcrEl1, width: BitWidth::W64 };
    let semantics = vec![
        Stmt::WriteReg { reg: Reg::X(1), value: val.clone() },
        Stmt::WriteReg { reg: Reg::X(2), value: val },
    ];

    Instruction {
        id: InstructionId { mnemonic: "MRRS".to_string(), encoding: Some("Move System register to two GPRs".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "mrrs.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn mrrs_validates() {
    validate_instruction(&mrrs_instruction()).expect("should validate");
}
