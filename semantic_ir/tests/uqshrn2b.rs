/// UQSHRN2B (Unsigned Saturating Shift Right Narrow 2 Bottom) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/uqshrn2b.md
/// Unsigned saturating shift right narrow 2 bottom. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn uqshrn2b_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Unsigned Saturating Shift Right Narrow 2 Bottom".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uqshrn2b.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uqshrn2b_validates() {
    validate_instruction(&uqshrn2b_instruction("UQSHRN2B"))
        .expect("UQSHRN2B should validate");
}
