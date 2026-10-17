/// LDAPURSH (Load-Acquire RCpc Register Signed Halfword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldapursh.md
/// Load-acquire RCpc signed halfword. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldapursh_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let data = Expr::MemRead {
        address: Box::new(address.clone()),
        size: MemSize::Bits16,
    };

    let value = Expr::SignExtend {
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
            encoding: Some("Load-Acquire RCpc Register Signed Halfword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldapursh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldapursh_validates() {
    validate_instruction(&ldapursh_instruction("LDAPURSH"))
        .expect("LDAPURSH should validate");
}
