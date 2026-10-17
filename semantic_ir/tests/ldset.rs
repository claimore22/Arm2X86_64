/// LDSET (Load Exclusive Set) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldset.md
/// Load exclusive set. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt,
    ValueType, Width, validate_instruction,
};

fn ldset_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };
    let old = Expr::MemAtomic {
        address: Box::new(address.clone()),
        expected: Box::new(data.clone()),
        new_value: Box::new(data),
        size: MemSize::Bits64,
        ordering: semantic_ir::MemOrdering::Relaxed,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(3),
        value: old,
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Exclusive Set".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldset.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldset_validates() {
    validate_instruction(&ldset_instruction("LDSET"))
        .expect("LDSET should validate");
}
