/// PSB CSYNC (Profiling synchronization barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/psb.md
///
/// ProfilingSynchronizationBarrier (FEAT_SPE).
/// Modeled as a placeholder Let statement (same precedent as CSDB/CLRBHB).
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn psb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "psb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PSB".to_string(),
            encoding: Some("PSB CSYNC (Profiling synchronization barrier)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "psb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn psb_validates() {
    validate_instruction(&psb_instruction()).expect("PSB should validate");
}

#[test]
fn psb_is_placeholder_operation() {
    let instruction = psb_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "psb"
    ));
}
