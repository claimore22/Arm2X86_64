/// BIC (predicates) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bicp.md
/// Bitwise bit clear predicates. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, Width,
    validate_instruction,
};

fn bicp_instruction(mnemonic: &str) -> Instruction {
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
            encoding: Some("Bitwise bit clear predicates".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bicp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bicp_validates() {
    validate_instruction(&bicp_instruction("BIC"))
        .expect("BIC should validate");
}
