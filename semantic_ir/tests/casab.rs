/// CASAB (Compare And Swap, Acquire, Bottom halfword).
/// Stanford source: arm64_md_clean/casab.md
/// Compare and swap acquire bottom halfword. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt,
    validate_instruction,
};

fn casab_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let expected = Expr::ReadReg {
        reg: Reg::W(2),
        width: BitWidth::W16,
    };
    let new = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W16,
    };

    let old = Expr::MemAtomic {
        address: Box::new(address),
        expected: Box::new(expected),
        new_value: Box::new(new),
        size: MemSize::Bits16,
        ordering: semantic_ir::MemOrdering::Relaxed,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(4),
        value: Expr::ZeroExtend {
            value: Box::new(old),
            to: BitWidth::W64,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Compare and swap acquire bottom halfword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "casab.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn casab_validates() {
    validate_instruction(&casab_instruction("CASAB"))
        .expect("CASAB should validate");
}
