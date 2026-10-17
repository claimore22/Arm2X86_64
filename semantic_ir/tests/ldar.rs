/// LDAR (Load-Acquire Register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldar.md
/// Load acquire register. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldar_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value: data,
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-Acquire Register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldar_validates() {
    validate_instruction(&cfvt_instruction("LDAR"))
        .expect("LDAR should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldar_instruction(mnemonic)
}