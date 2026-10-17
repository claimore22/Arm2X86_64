/// LDAXRH (Load-Acquire Exclusive Halfword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldaxrh.md
/// Load acquire exclusive halfword. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldaxrh_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits16,
    };
    let value = Expr::ZeroExtend {
        value: Box::new(data),
        to: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value,
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-Acquire Exclusive Halfword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldaxrh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldaxrh_validates() {
    validate_instruction(&cfvt_instruction("LDAXRH"))
        .expect("LDAXRH should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldaxrh_instruction(mnemonic)
}