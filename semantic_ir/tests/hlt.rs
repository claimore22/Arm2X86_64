/// HLT (Halt instruction) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/hlt.md
///
/// Halt(DebugHalt_HaltInstruction): opaque debug entry; UNDEFINED when HDE==0. Modeled as a placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn hlt_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "hlt".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "HLT".to_string(),
            encoding: Some("Halt instruction".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "hlt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn hlt_validates() {
    let instruction = hlt_instruction();
    validate_instruction(&instruction).expect("HLT should validate");
}

#[test]
fn hlt_is_placeholder_operation() {
    let instruction = hlt_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "hlt"
    ));
}
