/// PACM (Pointer authentication modifier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacm.md
///
/// Opaque PSTATE.PACM update (FEAT_PAuth_LR).
/// Modeled as a placeholder Let statement (same precedent as CSDB/CLRBHB).
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn pacm_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "pacm".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PACM".to_string(),
            encoding: Some("PACM (Pointer authentication modifier)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacm_validates() {
    validate_instruction(&pacm_instruction()).expect("PACM should validate");
}

#[test]
fn pacm_is_placeholder_operation() {
    let instruction = pacm_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "pacm"
    ));
}
