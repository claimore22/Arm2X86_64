/// GCSSTR (Guarded Control Stack store register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gcsstr.md
///
/// Stores a doubleword from Xt to memory at the address in Xn (or SP when
/// n == 31).  Requires FEAT_GCS.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance, Reg,
    Stmt, validate_instruction,
};

fn gcsstr_instruction() -> Instruction {
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
            mnemonic: "GCSSTR".to_string(),
            encoding: Some("Guarded Control Stack store register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_GCS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gcsstr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gcsstr_validates() {
    let instruction = gcsstr_instruction();
    validate_instruction(&instruction).expect("GCSSTR should validate");
}

#[test]
fn gcsstr_requires_feat_gcs() {
    assert!(gcsstr_instruction()
        .required_features
        .contains(&"FEAT_GCS".to_string()));
}

#[test]
fn gcsstr_stores_doubleword_to_indirect_address() {
    let instruction = gcsstr_instruction();

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
