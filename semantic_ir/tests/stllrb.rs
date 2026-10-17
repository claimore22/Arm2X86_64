/// STLLRB (Store LORelease register byte) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stllrb.md
///
/// Store LORelease register byte: MemWrite with release ordering.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stllrb_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let address = base;
    let semantics = vec![Stmt::MemWrite {
        address,
        value: Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W8 },
        size: MemSize::Bits8,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    }];

    Instruction {
        id: InstructionId { mnemonic: "STLLRB".to_string(), encoding: Some("Store LORelease register byte".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LOR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stllrb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stllrb_validates() {
    validate_instruction(&stllrb_instruction()).expect("should validate");
}

#[test]
fn stllrb_is_release_store() {
    let instruction = stllrb_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
