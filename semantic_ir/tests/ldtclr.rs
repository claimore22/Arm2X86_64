/// LDTCLR (Load Exclusive Clear) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldtclr.md
/// Load exclusive clear. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};
use semantic_ir::primitives::AtomicRMWOp;

fn ldtclr_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(data),
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
            encoding: Some("Load Exclusive Clear".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldtclr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldtclr_validates() {
    validate_instruction(&cfvt_instruction("LDTCLR"))
        .expect("LDTCLR should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldtclr_instruction(mnemonic)
}