/// ESB (Error Synchronization Barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/esb.md
///
/// Error synchronization event (`SynchronizeErrors()` + ESB/vESB/dESB
/// operations); if FEAT_RAS is not implemented it executes as a NOP.
/// Modeled as a placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn esb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "esb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ESB".to_string(),
            encoding: Some("Error synchronization barrier".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_RAS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "esb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn esb_validates() {
    let instruction = esb_instruction();
    validate_instruction(&instruction).expect("ESB should validate");
}

#[test]
fn esb_requires_feat_ras() {
    let instruction = esb_instruction();
    assert!(instruction.required_features.contains(&"FEAT_RAS".to_string()));
}

#[test]
fn esb_is_barrier_operation() {
    let instruction = esb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "esb"
    ));
}
