/// GCSSTTR (Guarded Control Stack store register (unprivileged)) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gcssttr.md
///
/// Stores a doubleword from Xt to memory at the address in Xn (or SP when
/// n == 31). Memory effects behave as if executed at EL0 under UAO conditions.  Requires FEAT_GCS.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance, Reg,
    Stmt, validate_instruction,
};

fn gcssttr_instruction() -> Instruction {
    // address = X(n) (n != 31 case)
    let address = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let value = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let semantics = vec![Stmt::MemWrite {
        address,
        value,
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "GCSSTTR".to_string(),
            encoding: Some("Guarded Control Stack store register (unprivileged)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_GCS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gcssttr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gcssttr_validates() {
    let instruction = gcssttr_instruction();
    validate_instruction(&instruction).expect("GCSSTTR should validate");
}

#[test]
fn gcssttr_requires_feat_gcs() {
    assert!(gcssttr_instruction()
        .required_features
        .contains(&"FEAT_GCS".to_string()));
}

#[test]
fn gcssttr_stores_doubleword_to_indirect_address() {
    let instruction = gcssttr_instruction();

    let Stmt::MemWrite {
        address,
        value,
        size,
..
    } = &instruction.semantics[0]
    else {
        panic!("expected MemWrite");
    };

    assert_eq!(*size, MemSize::Bits64);
    assert!(matches!(address, Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(value, Expr::ReadReg { reg: Reg::X(1), .. }));
}
