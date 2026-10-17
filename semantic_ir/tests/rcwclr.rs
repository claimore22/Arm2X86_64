/// RCWCLR, RCWCLRA, RCWCLRAL, RCWCLRL (RCW atomic bit clear, doubleword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwclr.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomicRMW`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwclr_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits64,
        ordering,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: old }];

    Instruction {
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write atomic bit clear on doubleword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwclr.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwclr_validates() {
    validate_instruction(&rcwclr_instruction("RCWCLR", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwclr_instruction("RCWCLRA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwclr_instruction("RCWCLRL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwclr_instruction("RCWCLRAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwclr_requires_feat_the() {
    assert!(rcwclr_instruction("RCWCLR", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwclr_uses_expected_node() {
    let instruction = rcwclr_instruction("RCWCLR", MemOrdering::Relaxed);
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::MemAtomicRMW { op: AtomicRMWOp::Bic, .. }));
}
