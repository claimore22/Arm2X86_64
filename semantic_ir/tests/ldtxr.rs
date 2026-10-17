/// LDTXR (Load Exclusive Tagged) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldtxr.md
/// Load exclusive tagged. Requires FEAT_LSE and FEAT_MTE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldt_xr_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::MemRead {
        address: Box::new(address.clone()),
        size: MemSize::Bits64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value: data,
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Exclusive Tagged".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string(), "FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldtxr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldtxr_validates() {
    validate_instruction(&cfvt_instruction("LDTXR"))
        .expect("LDTXR should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldt_xr_instruction(mnemonic)
}