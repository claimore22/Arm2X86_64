/// STUMAXH, STUMAXLH: Atomic unsigned maximum on halfword, without return semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stumaxh_ldumaxh.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn stumaxh_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W16 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W16 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Umax,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits16,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STUMAXH".to_string(), encoding: Some("Atomic unsigned maximum on halfword, without return (STUMAXH)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stumaxh_ldumaxh.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stumaxh_validates() {
    validate_instruction(&stumaxh_instruction()).expect("should validate");
}

#[test]
fn stumaxh_discards_result() {
    let instruction = stumaxh_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Umax, .. } }
    ));
}

