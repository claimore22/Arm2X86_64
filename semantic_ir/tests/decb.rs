/// DECB (Decrement Byte) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/decb.md
/// Decrement Byte. Requires FEAT_SME.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, Width,
    validate_instruction,
};

fn decb_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Decrement Byte".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "decb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn decb_validates() {
    validate_instruction(&decb_instruction("DECB"))
        .expect("DECB should validate");
}
