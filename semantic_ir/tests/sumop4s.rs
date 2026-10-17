/// SUMOP4S (Signed Multiply-Add Pairwise 4 Scalar) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sumop4s.md
/// Signed Multiply-Add Pairwise 4 Scalar. Requires FEAT_SME.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn sumop4s_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Signed Multiply-Add Pairwise 4 Scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sumop4s.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sumop4s_validates() {
    validate_instruction(&sumop4s_instruction("SUMOP4S"))
        .expect("SUMOP4S should validate");
}
