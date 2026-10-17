/// PSSBB (Physical speculative store bypass barrier: alias of DSB) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pssbb_dsb.md
///
/// SpeculativeStoreBypassBarrierToPA (DSB alias). FEAT_SPECRES2.
/// Modeled as a placeholder Let statement (same precedent as CSDB/CLRBHB).
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn pssbb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "pssbb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PSSBB".to_string(),
            encoding: Some("PSSBB (Physical speculative store bypass barrier: alias of DSB)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPECRES2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pssbb_dsb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pssbb_validates() {
    validate_instruction(&pssbb_instruction()).expect("PSSBB should validate");
}

#[test]
fn pssbb_is_placeholder_operation() {
    let instruction = pssbb_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "pssbb"
    ));
}
