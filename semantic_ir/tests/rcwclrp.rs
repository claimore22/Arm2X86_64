/// RCWCLRP, RCWCLRPA, RCWCLRPAL, RCWCLRPL (RCW atomic bit clear, quadword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwclrp.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomicRMW`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE (plus FEAT_D128).
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwclrp_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::Concat {
        high: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        low: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        width: BitWidth::W128,
    };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits128,
        ordering,
    };
    let lo = Expr::Extract { value: Box::new(old.clone()), lsb: 0, width: BitWidth::W64 };
    let hi = Expr::Extract { value: Box::new(old), lsb: 64, width: BitWidth::W64 };
    let semantics = vec![
        Stmt::WriteReg { reg: Reg::X(4), value: lo },
        Stmt::WriteReg { reg: Reg::X(5), value: hi },
    ];

    Instruction {
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write atomic bit clear on quadword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string(), "FEAT_D128".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwclrp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwclrp_validates() {
    validate_instruction(&rcwclrp_instruction("RCWCLRP", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwclrp_instruction("RCWCLRPA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwclrp_instruction("RCWCLRPL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwclrp_instruction("RCWCLRPAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwclrp_requires_feat_the() {
    assert!(rcwclrp_instruction("RCWCLRP", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwclrp_uses_expected_node() {
    let instruction = rcwclrp_instruction("RCWCLRP", MemOrdering::Relaxed);
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Extract { value, .. } = value else {
        panic!("expected Extract");
    };    assert!(matches!(value.as_ref(), Expr::MemAtomicRMW { op: AtomicRMWOp::Bic, .. }));
}
