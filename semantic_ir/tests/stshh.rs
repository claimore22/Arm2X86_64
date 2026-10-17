/// STSHH (Store shared hint) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stshh.md
///
/// Hint_StoreShared(stream): memory-clash hint; placeholder Let
/// (same precedent as CSDB). Requires FEAT_PCDPHINT.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn stshh_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "stshh".to_string(),
        value: Expr::Const { value: 1, ty: semantic_ir::BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "STSHH".to_string(), encoding: Some("Store shared hint".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_PCDPHINT".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stshh.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stshh_validates() {
    validate_instruction(&stshh_instruction()).expect("should validate");
}

#[test]
fn stshh_requires_pcdphint() {
    assert!(stshh_instruction().required_features.contains(&"FEAT_PCDPHINT".to_string()));
}
