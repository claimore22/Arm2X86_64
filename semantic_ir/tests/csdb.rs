/// CSDB (Consumption of Speculative Data Barrier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/csdb.md
///
/// CSDB is a memory barrier that controls speculative execution arising from
/// data value prediction. Operation: `ConsumptionOfSpeculativeDataBarrier()`.
/// In the IR we model it as a placeholder Let statement, same precedent as CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn csdb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "csdb".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CSDB".to_string(),
            encoding: Some("Consumption of speculative data barrier".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "csdb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn csdb_validates() {
    let instruction = csdb_instruction();
    validate_instruction(&instruction).expect("CSDB should validate");
}

#[test]
fn csdb_is_barrier_operation() {
    let instruction = csdb_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "csdb"
    ));
}
