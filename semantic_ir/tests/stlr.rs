/// STLR (Store-release register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stlr.md
///
/// Store-release register: MemWrite with release ordering.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stlr_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let address = base;
    let semantics = vec![Stmt::MemWrite {
        address,
        value: Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 },
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    }];

    Instruction {
        id: InstructionId { mnemonic: "STLR".to_string(), encoding: Some("Store-release register".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stlr.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stlr_validates() {
    validate_instruction(&stlr_instruction()).expect("should validate");
}

#[test]
fn stlr_is_release_store() {
    let instruction = stlr_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
