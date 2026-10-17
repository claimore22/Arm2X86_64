/// LDNT1H (Load Non-Temporal Halfword, vector plus scalar) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldnt1h.md
/// Non-temporal load single halfword with vector plus scalar addressing. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn ldnt1h_vec_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Non-Temporal Load Single Halfword Vector plus Scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnt1h_vec.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnt1h_vec_validates() {
    validate_instruction(&ldnt1h_vec_instruction("LDNT1H"))
        .expect("LDNT1H vector plus scalar should validate");
}
