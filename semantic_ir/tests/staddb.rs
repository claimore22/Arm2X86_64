/// STADDB, STADDLB: Atomic add on byte, without return (LDADDB alias) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/staddb_ldaddb.md
///
/// Alias of the corresponding LD* atomic without a return value (result
/// discarded). Modeled with `Expr::MemAtomicRMW` + Stmt::Discard.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn staddb_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let value = Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W8 };
    let op = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Add,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits8,
        ordering: MemOrdering::Relaxed,
    };
    let semantics = vec![Stmt::Discard { value: op }];

    Instruction {
        id: InstructionId { mnemonic: "STADDB".to_string(), encoding: Some("Atomic add on byte, without return (LDADDB alias)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "staddb_ldaddb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn staddb_validates() {
    validate_instruction(&staddb_instruction()).expect("should validate");
}

#[test]
fn staddb_discards_result() {
    let instruction = staddb_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemAtomicRMW { op: AtomicRMWOp::Add, .. } }
    ));
}
