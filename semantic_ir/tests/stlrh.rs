/// STLRH (Store-release register halfword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stlrh.md
///
/// Store-release register halfword: MemWrite with release ordering.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stlrh_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let address = base;
    let semantics = vec![Stmt::MemWrite {
        address,
        value: Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W16 },
        size: MemSize::Bits16,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    }];

    Instruction {
        id: InstructionId { mnemonic: "STLRH".to_string(), encoding: Some("Store-release register halfword".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stlrh.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stlrh_validates() {
    validate_instruction(&stlrh_instruction()).expect("should validate");
}

#[test]
fn stlrh_is_release_store() {
    let instruction = stlrh_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
