/// BLRAB (Branch with Link to Register, Authenticated, using key B).
/// Stanford source: arm64_md_clean/blrab.md
/// Branch with link to register, authenticated using key B. Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, Width,
    validate_instruction,
};

fn blrab_instruction(mnemonic: &str) -> Instruction {
    let data = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: data,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Branch with link to register, authenticated using key B".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "blrab.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn blrab_validates() {
    validate_instruction(&blrab_instruction("BLRAB"))
        .expect("BLRAB should validate");
}
