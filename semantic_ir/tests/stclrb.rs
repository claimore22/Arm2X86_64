/// STCLRB, STCLRLB: Atomic bit clear on byte, without return (LDCLRB alias) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stclrb_ldclrb.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn stclrb_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W8 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits8,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STCLRB".to_string(), encoding: Some("Atomic bit clear on byte, without return (LDCLRB alias)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stclrb_ldclrb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stclrb_validates() {
    validate_instruction(&stclrb_instruction()).expect("should validate");
}

#[test]
fn stclrb_discards_result() {
    let instruction = stclrb_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Bic, .. } }
    ));
}
