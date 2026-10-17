/// DCPS1 (Debug change PE state to EL1) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dcps1.md
///
/// Only defined in Debug state; in Non-debug state it is UNDEFINED.
/// Operation: `DCPSInstruction(EL1)` - an opaque debug-state transition
/// (ELR_ELx/SPSR_ELx/ESR_ELx/DLR_EL0/DSPSR_EL0 become UNKNOWN, SP selected
/// per target EL). Modeled as a placeholder Let statement, same precedent
/// as CSDB/CLRBHB barrier/hint instructions.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn dcps1_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "dcps1".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DCPS1".to_string(),
            encoding: Some("Debug change PE state to EL1".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dcps1.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dcps1_validates() {
    let instruction = dcps1_instruction();
    validate_instruction(&instruction).expect("DCPS1 should validate");
}

#[test]
fn dcps1_is_debug_state_operation() {
    let instruction = dcps1_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "dcps1"
    ));
}
