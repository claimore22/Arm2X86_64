/// LDG (Load Tag) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldg.md
/// Load allocation tag from memory. Requires FEAT_MTE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldg_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let tag = Expr::MemTagRead {
        address: Box::new(address),
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value: Expr::ZeroExtend {
            value: Box::new(tag),
            to: BitWidth::W64,
        },
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Tag".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldg_validates() {
    validate_instruction(&cfvt_instruction("LDG"))
        .expect("LDG should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldg_instruction(mnemonic)
}