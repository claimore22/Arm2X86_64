/// DMB (Data memory barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dmb.md
///
/// Barrier/hint instruction with an opaque architectural operation
/// (no register or flag effects in pseudocode form). Modeled as a
/// placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn dmb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "dmb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DMB".to_string(),
            encoding: Some("Data memory barrier".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dmb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dmb_validates() {
    let instruction = dmb_instruction();
    validate_instruction(&instruction).expect("DMB should validate");
}

#[test]
fn dmb_is_barrier_or_hint() {
    let instruction = dmb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "dmb"
    ));
}


