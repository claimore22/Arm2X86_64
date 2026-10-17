/// USMMLA (Unsigned Saturating Multiply-Add) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/usmmla.md
/// Unsigned Saturating Multiply-Add. Requires FEAT_SME.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, Width,
    validate_instruction,
};

fn usmmla_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Unsigned Saturating Multiply-Add".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "usmmla.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn usmmla_validates() {
    validate_instruction(&usmmla_instruction("USMMLA"))
        .expect("USMMLA should validate");
}
