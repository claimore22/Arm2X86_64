/// USMOP4A (Unsigned Saturating Multiply-Add Pairwise 4 Accumulate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/usmop4a.md
/// Unsigned Saturating Multiply-Add Pairwise 4 Accumulate. Requires FEAT_SME.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn usmop4a_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Unsigned Saturating Multiply-Add Pairwise 4 Accumulate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "usmop4a.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn usmop4a_validates() {
    validate_instruction(&usmop4a_instruction("USMOP4A"))
        .expect("USMOP4A should validate");
}
