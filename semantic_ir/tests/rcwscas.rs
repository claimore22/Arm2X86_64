/// RCWSCAS, RCWSCASA, RCWSCASAL, RCWSCASL (RCW software compare and swap, doubleword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rcwscas.md
///
/// Read Check Write (RCW) operation, modeled with `Expr::MemAtomic`
/// (the read-check is access bookkeeping handled in lowering).
/// Requires FEAT_THE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn rcwscas_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let expected = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let new_value = Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 };
    let old = Expr::MemAtomic {
        address: Box::new(address),
        expected: Box::new(expected),
        new_value: Box::new(new_value),
        size: MemSize::Bits64,
        ordering,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: old }];

    Instruction {
        id: InstructionId { mnemonic: mnemonic.to_string(), encoding: Some("Read check write software compare and swap doubleword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_THE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rcwscas.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rcwscas_validates() {
    validate_instruction(&rcwscas_instruction("RCWSCAS", MemOrdering::Relaxed)).expect("v");
    validate_instruction(&rcwscas_instruction("RCWSCASA", MemOrdering::Acquire)).expect("v");
    validate_instruction(&rcwscas_instruction("RCWSCASL", MemOrdering::Release)).expect("v");
    validate_instruction(&rcwscas_instruction("RCWSCASAL", MemOrdering::AcquireRelease)).expect("v");
}

#[test]
fn rcwscas_requires_feat_the() {
    assert!(rcwscas_instruction("RCWSCAS", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_THE".to_string()));
}

#[test]
fn rcwscas_uses_expected_node() {
    let instruction = rcwscas_instruction("RCWSCAS", MemOrdering::Relaxed);
            let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::MemAtomic { .. }));
}
