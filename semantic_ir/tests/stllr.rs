/// STLLR (Store LORelease register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stllr.md
///
/// Store LORelease register: MemWrite with release ordering (LOAcquire/LORelease, FEAT_LOR).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stllr_instruction() -> Instruction {
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
        id: InstructionId { mnemonic: "STLLR".to_string(), encoding: Some("Store LORelease register".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LOR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stllr.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stllr_validates() {
    validate_instruction(&stllr_instruction()).expect("should validate");
}

#[test]
fn stllr_is_release_store() {
    let instruction = stllr_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
