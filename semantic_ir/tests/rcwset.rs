/// RCWSET, RCWSETA, RCWSETAL, RCWSETL (RCW atomic bit set, doubleword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwset.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomicRMW`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwset_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Orr,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits64,
        ordering,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: old }];

    Instruction {
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write atomic bit set on doubleword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwset.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwset_validates() {
    validate_instruction(&rcwset_instruction("RCWSET", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwset_instruction("RCWSETA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwset_instruction("RCWSETL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwset_instruction("RCWSETAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwset_requires_feat_the() {
    assert!(rcwset_instruction("RCWSET", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwset_uses_expected_node() {
    let instruction = rcwset_instruction("RCWSET", MemOrdering::Relaxed);
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::MemAtomicRMW { op: AtomicRMWOp::Orr, .. }));
}
