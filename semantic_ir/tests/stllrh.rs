/// STLLRH (Store LORelease register halfword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stllrh.md
///
/// Store LORelease register halfword: MemWrite with release ordering.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stllrh_instruction() -> Instruction {
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
        id: InstructionId { mnemonic: "STLLRH".to_string(), encoding: Some("Store LORelease register halfword".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LOR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stllrh.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stllrh_validates() {
    validate_instruction(&stllrh_instruction()).expect("should validate");
}

#[test]
fn stllrh_is_release_store() {
    let instruction = stllrh_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
