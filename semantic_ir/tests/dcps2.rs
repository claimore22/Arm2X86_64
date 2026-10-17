/// DCPS2 (Debug change PE state to EL2) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dcps2.md
///
/// Only defined in Debug state; in Non-debug state it is UNDEFINED.
/// Operation: `DCPSInstruction(EL2)` - an opaque debug-state transition
/// (ELR_ELx/SPSR_ELx/ESR_ELx/DLR_EL0/DSPSR_EL0 become UNKNOWN, SP selected
/// per target EL). Modeled as a placeholder Let statement, same precedent
/// as CSDB/CLRBHB barrier/hint instructions.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn dcps2_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "dcps2".to_string(),
        value: Expr::Const {
            value: 2,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DCPS2".to_string(),
            encoding: Some("Debug change PE state to EL2".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dcps2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dcps2_validates() {
    let instruction = dcps2_instruction();
    validate_instruction(&instruction).expect("DCPS2 should validate");
}

#[test]
fn dcps2_is_debug_state_operation() {
    let instruction = dcps2_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "dcps2"
    ));
}
