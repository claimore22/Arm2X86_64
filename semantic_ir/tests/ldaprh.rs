/// LDAPRH (Load-Acquire Register Pair Halfword) semantics.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldaprh_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits16,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value: Expr::ZeroExtend {
            value: Box::new(data),
            to: BitWidth::W64,
        },
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-Acquire Register Pair Halfword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldaprh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldaprh_validates() {
    validate_instruction(&cfvt_instruction("LDAPRH"))
        .expect("LDAPRH should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldaprh_instruction(mnemonic)
}