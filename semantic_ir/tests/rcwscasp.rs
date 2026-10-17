/// RCWSCASP, RCWSCASPA, RCWSCASPAL, RCWSCASPL (RCW software compare and swap, quadword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwscasp.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomic`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE (plus FEAT_D128).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwscasp_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    // 128-bit expected/new values are register pairs
    let expected = Expr::Concat {
        high: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        low: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        width: BitWidth::W128,
    };
    let new_value = Expr::Concat {
        high: Box::new(Expr::ReadReg { reg: Reg::X(5), width: BitWidth::W64 }),
        low: Box::new(Expr::ReadReg { reg: Reg::X(4), width: BitWidth::W64 }),
        width: BitWidth::W128,
    };
    let old = Expr::MemAtomic {
        address: Box::new(address),
        expected: Box::new(expected),
        new_value: Box::new(new_value),
        size: MemSize::Bits128,
        ordering,
    };
    let lo = Expr::Extract { value: Box::new(old.clone()), lsb: 0, width: BitWidth::W64 };
    let hi = Expr::Extract { value: Box::new(old), lsb: 64, width: BitWidth::W64 };
    let semantics = vec![
        Stmt::WriteReg { reg: Reg::X(0), value: lo },
        Stmt::WriteReg { reg: Reg::X(1), value: hi },
    ];

    Instruction {
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write software compare and swap quadword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string(), "FEAT_D128".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwscasp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwscasp_validates() {
    validate_instruction(&rcwscasp_instruction("RCWSCASP", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwscasp_instruction("RCWSCASPA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwscasp_instruction("RCWSCASPL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwscasp_instruction("RCWSCASPAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwscasp_requires_feat_the() {
    assert!(rcwscasp_instruction("RCWSCASP", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwscasp_uses_expected_node() {
    let instruction = rcwscasp_instruction("RCWSCASP", MemOrdering::Relaxed);
                let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Extract { value, .. } = value else {
        panic!("expected Extract");
    };
    assert!(matches!(value.as_ref(), Expr::MemAtomic { .. }));
}

