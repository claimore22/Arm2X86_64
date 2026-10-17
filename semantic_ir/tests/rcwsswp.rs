/// RCWSSWP, RCWSSWPA, RCWSSWPAL, RCWSSWPL (RCW software swap, doubleword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwsswp.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomicRMW`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwsswp_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Swp,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits64,
        ordering,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: old }];

    Instruction {
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write software swap doubleword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwsswp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwsswp_validates() {
    validate_instruction(&rcwsswp_instruction("RCWSSWP", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwsswp_instruction("RCWSSWPA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwsswp_instruction("RCWSSWPL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwsswp_instruction("RCWSSWPAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwsswp_requires_feat_the() {
    assert!(rcwsswp_instruction("RCWSSWP", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwsswp_uses_expected_node() {
    let instruction = rcwsswp_instruction("RCWSSWP", MemOrdering::Relaxed);
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::MemAtomicRMW { op: AtomicRMWOp::Swp, .. }));
}
