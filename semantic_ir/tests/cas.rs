/// CAS (Compare And Swap) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cas.md
/// CAS, CASA, CASAL, CASL - Compare and swap.
/// Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt,
    ValueType, Width, validate_instruction,
};
use semantic_ir::primitives::AtomicRMWOp;

fn cas_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let expected = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };
    let new = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    let old = Expr::MemAtomic {
        address: Box::new(address),
        expected: Box::new(expected),
        new_value: Box::new(new),
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
            encoding: Some("Compare And Swap".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cas.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cas_validates() {
    validate_instruction(&cfvt_instruction("CAS")).expect("CAS should validate");
}

fn cfvt_instruction(mnemonic: &str) -> Instruction {
    cas_instruction(mnemonic)
}