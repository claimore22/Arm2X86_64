/// STTADD, STTADDL: Atomic add unprivileged, without return (LDTADD alias) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sttadd_ldtadd.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn sttadd_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W64 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Add,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits64,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STTADD".to_string(), encoding: Some("Atomic add unprivileged, without return (LDTADD alias) (STTADD)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sttadd_ldtadd.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sttadd_validates() {
    validate_instruction(&sttadd_instruction()).expect("should validate");
}

#[test]
fn sttadd_discards_result() {
    let instruction = sttadd_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Add, .. } }
    ));
}

