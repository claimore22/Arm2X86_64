/// STEOR, STEORL: Atomic exclusive-OR on word or doubleword, without return (LDEOR alias) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/steor_ldeor.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn steor_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W64 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Eor,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits64,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STEOR".to_string(), encoding: Some("Atomic exclusive-OR on word or doubleword, without return (LDEOR alias)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "steor_ldeor.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn steor_validates() {
    validate_instruction(&steor_instruction()).expect("should validate");
}

#[test]
fn steor_discards_result() {
    let instruction = steor_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Eor, .. } }
    ));
}
