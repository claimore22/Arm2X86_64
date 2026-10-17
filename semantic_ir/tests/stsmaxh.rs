/// STSMAXH, STSMAXLH: Atomic signed maximum on halfword, without return semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stsmaxh_ldsmaxh.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn stsmaxh_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W16 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W16 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Smax,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits16,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STSMAXH".to_string(), encoding: Some("Atomic signed maximum on halfword, without return (STSMAXH)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stsmaxh_ldsmaxh.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stsmaxh_validates() {
    validate_instruction(&stsmaxh_instruction()).expect("should validate");
}

#[test]
fn stsmaxh_discards_result() {
    let instruction = stsmaxh_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Smax, .. } }
    ));
}

