/// CLRBHB (Clear Branch History) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/clrbhb.md
use semantic_ir::{
    Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn clrbhb_instruction() -> Instruction {
    // CLRBHB is a hint instruction that clears branch history.
    // It requires FEAT_CLRBHB. In the IR, we model it as a Let statement
    // to preserve the hint operation (similar to BTI).
    let semantics = vec![Stmt::Let {
        name: "clrbhb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CLRBHB".to_string(),
            encoding: Some("Clear branch history".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CLRBHB".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "clrbhb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn clrbhb_validates() {
    let instruction = clrbhb_instruction();

    validate_instruction(&instruction).expect("CLRBHB should validate");
}

#[test]
fn clrbhb_requires_clrbhb_feature() {
    let instruction = clrbhb_instruction();

    assert!(instruction.required_features.contains(&"FEAT_CLRBHB".to_string()));
}

#[test]
fn clrbhb_is_hint_operation() {
    let instruction = clrbhb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "clrbhb"
    ));
}