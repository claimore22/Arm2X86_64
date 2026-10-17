/// DCPS3 (Debug change PE state to EL3) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dcps3.md
///
/// Only defined in Debug state; in Non-debug state it is UNDEFINED.
/// Operation: `DCPSInstruction(EL3)` - an opaque debug-state transition
/// (ELR_ELx/SPSR_ELx/ESR_ELx/DLR_EL0/DSPSR_EL0 become UNKNOWN, SP selected
/// per target EL). Modeled as a placeholder Let statement, same precedent
/// as CSDB/CLRBHB barrier/hint instructions.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn dcps3_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "dcps3".to_string(),
        value: Expr::Const {
            value: 3,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DCPS3".to_string(),
            encoding: Some("Debug change PE state to EL3".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dcps3.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dcps3_validates() {
    let instruction = dcps3_instruction();
    validate_instruction(&instruction).expect("DCPS3 should validate");
}

#[test]
fn dcps3_is_debug_state_operation() {
    let instruction = dcps3_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "dcps3"
    ));
}
