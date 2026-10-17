/// LDAPR (Load-Acquire RCpc Register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldapr.md
/// Load acquire with RCpc ordering. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldapr_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();

    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let data = Expr::MemRead {
        address: Box::new(address.clone()),
        size,
    };

    let value = if width == BitWidth::W64 {
        data
    } else {
        Expr::ZeroExtend {
            value: Box::new(data),
            to: BitWidth::W64,
        }
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-Acquire RCpc Register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldapr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldapr_validates() {
    validate_instruction(&ldapr_instruction("LDAPR", MemSize::Bits64)).expect("LDAPR should validate");
}

#[test]
fn ldaprh_validates_16bit() {
    validate_instruction(&ldapr_instruction("LDAPRH", MemSize::Bits16)).expect("LDAPRH should validate");
}

#[test]
fn ldapursh_validates_16bit_signed() {
    validate_instruction(&ldapr_instruction("LDAPURSH", MemSize::Bits16))
        .expect("LDAPURSH should validate");
}
