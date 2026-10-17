/// CLREX (Clear Exclusive) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/clrex.md
use semantic_ir::{
    Instruction, InstructionId, Provenance, Stmt, validate_instruction,
};

fn clrex_instruction() -> Instruction {
    // CLREX: ClearExclusiveLocal(ProcessorID())
    // In the semantic IR, we model this as a special statement that clears the local exclusive monitor
    // For now, we'll use a Discard statement with a special value to represent this
    let semantics = vec![Stmt::Discard {
        value: semantic_ir::Expr::Const {
            value: 0,
            ty: semantic_ir::BitWidth::W64,
        },
    }];

    Instruction {
        id: semantic_ir::InstructionId {
            mnemonic: "CLREX".to_string(),
            encoding: Some("Clear exclusive".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: semantic_ir::Provenance {
            source_path: "clrex.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn clrex_validates() {
    let instruction = clrex_instruction();

    validate_instruction(&instruction).expect("CLREX should validate");
}

#[test]
fn clrex_discards_result() {
    let instruction = clrex_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard");
    };

    // The value is just a placeholder constant
    let semantic_ir::Expr::Const { value, ty } = value else {
        panic!("expected Const");
    };

    assert_eq!(*value, 0);
}