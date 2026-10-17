/// DGH (Data gathering hint) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dgh.md
///
/// Barrier/hint instruction with an opaque architectural operation
/// (no register or flag effects in pseudocode form). Modeled as a
/// placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn dgh_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "dgh".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DGH".to_string(),
            encoding: Some("Data gathering hint".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_DGH".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dgh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dgh_validates() {
    let instruction = dgh_instruction();
    validate_instruction(&instruction).expect("DGH should validate");
}

#[test]
fn dgh_is_barrier_or_hint() {
    let instruction = dgh_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "dgh"
    ));
}

#[test]
fn dgh_requires_feat_dgh() {
    let instruction = dgh_instruction();
    assert!(instruction
        .required_features
        .contains(&"FEAT_DGH".to_string()));
}
