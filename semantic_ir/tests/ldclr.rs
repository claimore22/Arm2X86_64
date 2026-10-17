/// LDCLR, LDCLRA, LDCLRAL, LDCLRL (Atomic bit clear on word or doubleword)
/// Stanford source: arm64_md_clean/ldclr.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, applies bit clear with Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Bic, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldclr_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
    let width = size.width();

    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let value = Expr::ReadReg {
        reg: if width == BitWidth::W64 { Reg::X(2) } else { Reg::W(2) },
        width,
    };

    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(value),
        size,
        ordering,
    };

    // if t != 31: X(t) = ZeroExtend(data)
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(3),
        value: if width == BitWidth::W64 {
            old
        } else {
            Expr::ZeroExtend {
                value: Box::new(old),
                to: BitWidth::W64,
            }
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Atomic bit clear".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldclr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldclr_validates() {
    validate_instruction(&ldclr_instruction("LDCLR", MemSize::Bits64, MemOrdering::Relaxed))
        .expect("LDCLR should validate");
}

#[test]
fn ldclra_validates_with_acquire() {
    validate_instruction(&ldclr_instruction("LDCLRA", MemSize::Bits64, MemOrdering::Acquire))
        .expect("LDCLR should validate");
}

#[test]
fn ldclral_validates_with_acquire_release() {
    validate_instruction(&ldclr_instruction(
        "LDCLRAL",
        MemSize::Bits64,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDCLR should validate");
}

#[test]
fn ldclrl_validates_with_release() {
    validate_instruction(&ldclr_instruction("LDCLRL", MemSize::Bits64, MemOrdering::Release))
        .expect("LDCLR should validate");
}

#[test]
fn ldclrb_and_ldclrh_validate() {
    validate_instruction(&ldclr_instruction("LDCLRB", MemSize::Bits8, MemOrdering::Relaxed))
        .expect("LDCLR should validate");
    validate_instruction(&ldclr_instruction("LDCLRH", MemSize::Bits16, MemOrdering::Relaxed))
        .expect("LDCLR should validate");
}

#[test]
fn ldclr_requires_feat_lse() {
    assert!(ldclr_instruction("LDCLR", MemSize::Bits64, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn ldclr_uses_atomic_rmw_op() {
    let instruction = ldclr_instruction("LDCLRAL", MemSize::Bits64, MemOrdering::AcquireRelease);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::MemAtomicRMW {
        op,
        address,
        value: newvalue,
..
    } = value
    else {
        panic!("expected MemAtomicRMW");
    };

    assert_eq!(*op, AtomicRMWOp::Bic);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}

