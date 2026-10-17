/// LDSETH (Load Exclusive Set Halfword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldseth.md
/// Load exclusive set halfword. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};
use semantic_ir::primitives::AtomicRMWOp;

fn ldseth_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::ReadReg {
        reg: Reg::W(2),
        width: BitWidth::W16,
    };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Orr,
        address: Box::new(address),
        value: Box::new(data),
        size: MemSize::Bits16,
        ordering: semantic_ir::MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(3),
        value: Expr::ZeroExtend {
            value: Box::new(old),
            to: BitWidth::W64,
        },
    }];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Exclusive Set Halfword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldseth.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldseth_validates() {
    validate_instruction(&cfvt_instruction("LDSETH")).expect("LDSETH should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldseth_instruction(mnemonic)
}