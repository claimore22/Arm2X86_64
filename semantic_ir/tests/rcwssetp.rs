/// RCWSSETP, RCWSSETPA, RCWSSETPAL, RCWSSETPL (RCW software atomic bit set, quadword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwssetp.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomicRMW`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE (plus FEAT_D128).
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwssetp_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::Concat {
        high: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        low: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        width: BitWidth::W128,
    };
    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Orr,
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
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write software atomic bit set on quadword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string(), "FEAT_D128".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwssetp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwssetp_validates() {
    validate_instruction(&rcwssetp_instruction("RCWSSETP", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwssetp_instruction("RCWSSETPA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwssetp_instruction("RCWSSETPL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwssetp_instruction("RCWSSETPAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwssetp_requires_feat_the() {
    assert!(rcwssetp_instruction("RCWSSETP", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwssetp_uses_expected_node() {
    let instruction = rcwssetp_instruction("RCWSSETP", MemOrdering::Relaxed);
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Extract { value, .. } = value else {
        panic!("expected Extract");
    };    assert!(matches!(value.as_ref(), Expr::MemAtomicRMW { op: AtomicRMWOp::Orr, .. }));
}
