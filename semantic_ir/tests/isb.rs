/// ISB (Instruction synchronization barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/isb.md
///
/// InstructionSynchronizationBarrier(): context synchronization event. Modeled as a placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn isb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "isb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ISB".to_string(),
            encoding: Some("Instruction synchronization barrier".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "isb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn isb_validates() {
    let instruction = isb_instruction();
    validate_instruction(&instruction).expect("ISB should validate");
}

#[test]
fn isb_is_placeholder_operation() {
    let instruction = isb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "isb"
    ));
}
