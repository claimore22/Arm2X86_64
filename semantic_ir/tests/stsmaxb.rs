/// STSMAXB, STSMAXLB: Atomic signed maximum on byte, without return semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stsmaxb_ldsmaxb.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn stsmaxb_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W8 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W8 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Smax,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits8,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STSMAXB".to_string(), encoding: Some("Atomic signed maximum on byte, without return (STSMAXB)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stsmaxb_ldsmaxb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stsmaxb_validates() {
    validate_instruction(&stsmaxb_instruction()).expect("should validate");
}

#[test]
fn stsmaxb_discards_result() {
    let instruction = stsmaxb_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Smax, .. } }
    ));
}

