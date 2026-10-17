/// STSETB, STSETLB: Atomic bit set on byte, without return (LDSETB alias) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stsetb_ldsetb.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn stsetb_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W8 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W8 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Orr,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits8,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STSETB".to_string(), encoding: Some("Atomic bit set on byte, without return (LDSETB alias) (STSETB)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stsetb_ldsetb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stsetb_validates() {
    validate_instruction(&stsetb_instruction()).expect("should validate");
}

#[test]
fn stsetb_discards_result() {
    let instruction = stsetb_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Orr, .. } }
    ));
}

