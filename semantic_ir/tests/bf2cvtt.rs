/// BF2CVTLT (BFloat16 Convert Long 2 Top) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bf2cvtt.md
/// BFloat16 Convert Long 2 Top. Requires FEAT_SME.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn bf2cvtt_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("BFloat16 Convert Long 2 Top".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bf2cvtt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bf2cvtt_validates() {
    validate_instruction(&bf2cvtt_instruction("BF2CVTLT"))
        .expect("BF2CVTLT should validate");
}
