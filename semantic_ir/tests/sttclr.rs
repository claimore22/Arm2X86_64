/// STTCLR, STTCLRL: Atomic bit clear unprivileged, without return (LDTCLR alias) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sttclr_ldtclr.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn sttclr_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W64 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits64,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STTCLR".to_string(), encoding: Some("Atomic bit clear unprivileged, without return (LDTCLR alias) (STTCLR)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sttclr_ldtclr.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sttclr_validates() {
    validate_instruction(&sttclr_instruction()).expect("should validate");
}

#[test]
fn sttclr_discards_result() {
    let instruction = sttclr_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Bic, .. } }
    ));
}

