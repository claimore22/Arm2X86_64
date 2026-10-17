/// DSB (Data synchronization barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dsb.md
///
/// Barrier/hint instruction with an opaque architectural operation
/// (no register or flag effects in pseudocode form). Modeled as a
/// placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn dsb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "dsb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DSB".to_string(),
            encoding: Some("Data synchronization barrier".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dsb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dsb_validates() {
    let instruction = dsb_instruction();
    validate_instruction(&instruction).expect("DSB should validate");
}

#[test]
fn dsb_is_barrier_or_hint() {
    let instruction = dsb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "dsb"
    ));
}


