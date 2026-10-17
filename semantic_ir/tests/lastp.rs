/// LASTP (Load Pair of Pointers) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/lastp.md
/// Load Pair of Pointers. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, Width,
    validate_instruction,
};

fn lastp_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let old = Expr::MemAtomic {
        address: Box::new(address),
        expected: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        new_value: Box::new(Expr::ReadReg {
            reg: Reg::X(3),
            width: BitWidth::W64,
        }),
        size: MemSize::Bits64,
        ordering: semantic_ir::MemOrdering::Relaxed,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(4),
        value: old,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Pair of Pointers".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lastp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lastp_validates() {
    validate_instruction(&lastp_instruction("LASTP"))
        .expect("LASTP should validate");
}
