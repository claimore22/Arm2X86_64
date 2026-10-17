/// GCSB (Guarded Control Stack barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gcsb.md
///
/// GCSSynchronizationBarrier(); NOP when FEAT_GCS is not implemented.
/// Modeled as a placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn gcsb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "gcsb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "GCSB".to_string(),
            encoding: Some("Guarded Control Stack barrier".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_GCS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gcsb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gcsb_validates() {
    let instruction = gcsb_instruction();
    validate_instruction(&instruction).expect("GCSB should validate");
}

#[test]
fn gcsb_requires_feat_gcs() {
    assert!(gcsb_instruction()
        .required_features
        .contains(&"FEAT_GCS".to_string()));
}

#[test]
fn gcsb_is_barrier_operation() {
    let instruction = gcsb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "gcsb"
    ));
}
