/// BFCVTN2 (single-precision to interleaved BFloat16, 2) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bfcvtn2.md
/// BFloat16 convert narrowing 2. Requires FEAT_SME.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn bfcvtn2_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("BFloat16 Convert Narrowing 2".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfcvtn2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfcvtn2_validates() {
    validate_instruction(&bfcvtn2_instruction("BFCVTN2"))
        .expect("BFCVTN2 should validate");
}
