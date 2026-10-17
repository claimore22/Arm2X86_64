/// HINT (Hint instruction) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/hint.md
///
/// SystemHintOp dispatch (NOP/YIELD/WFE/WFI/SEV/SEVL/etc.); unallocated encodings behave as NOP. Modeled as a placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn hint_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "hint".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "HINT".to_string(),
            encoding: Some("Hint instruction".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "hint.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn hint_validates() {
    let instruction = hint_instruction();
    validate_instruction(&instruction).expect("HINT should validate");
}

#[test]
fn hint_is_placeholder_operation() {
    let instruction = hint_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "hint"
    ));
}
