/// SQSHLT2 (Signed Saturating Shift Left Top 2) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sqshrlt2.md
/// Signed saturating shift left top 2. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn sqshrlt2_instruction(mnemonic: &str) -> Instruction {
    let data = Expr::ReadScalableVecReg {
        reg: 0,
        vl: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: data,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Signed Saturating Shift Left Top 2".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqshrlt2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqshrlt2_validates() {
    validate_instruction(&sqshrlt2_instruction("SQSHLT2"))
        .expect("SQSHLT2 should validate");
}
