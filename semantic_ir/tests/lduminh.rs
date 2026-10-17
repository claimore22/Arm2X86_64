/// LDUMINH, LDUMINAH, LDUMINALH, LDUMINLH (Atomic unsigned minimum on halfword)
/// Stanford source: arm64_md_clean/lduminh.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, applies unsigned minimum with Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Umin, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn lduminh_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Umin,
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
            encoding: Some("Atomic unsigned minimum".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lduminh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lduminh_validates() {
    validate_instruction(&lduminh_instruction("LDUMINHB", MemSize::Bits16, MemOrdering::Relaxed))
        .expect("LDUMINH should validate");
}

#[test]
fn lduminha_validates_with_acquire() {
    validate_instruction(&lduminh_instruction("LDUMINAH", MemSize::Bits16, MemOrdering::Acquire))
        .expect("LDUMINH should validate");
}

#[test]
fn lduminhal_validates_with_acquire_release() {
    validate_instruction(&lduminh_instruction(
        "LDUMINALH",
        MemSize::Bits16,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDUMINH should validate");
}

#[test]
fn lduminhl_validates_with_release() {
    validate_instruction(&lduminh_instruction("LDUMINLH", MemSize::Bits16, MemOrdering::Release))
        .expect("LDUMINH should validate");
}

#[test]
fn lduminh_subword_variants_validate() {
    validate_instruction(&lduminh_instruction("LDUMINHAB", MemSize::Bits8, MemOrdering::Acquire))
        .expect("LDUMINH B-variant should validate");
    validate_instruction(&lduminh_instruction("LDUMINHALH", MemSize::Bits16, MemOrdering::AcquireRelease))
        .expect("LDUMINH H-variant should validate");
}
#[test]
fn lduminh_requires_feat_lse() {
    assert!(lduminh_instruction("LDUMINHB", MemSize::Bits16, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn lduminh_uses_atomic_rmw_op() {
    let instruction = lduminh_instruction("LDUMINALH", MemSize::Bits16, MemOrdering::AcquireRelease);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // sub-word results are zero-extended to 64-bit in the destination
    let inner = match value {
        Expr::ZeroExtend { value: inner, .. } => inner.as_ref(),
        other => other,
    };

    let Expr::MemAtomicRMW {
        op,
        address,
        value: newvalue,
..
    } = inner
    else {
        panic!("expected MemAtomicRMW");
    };

    assert_eq!(*op, AtomicRMWOp::Umin);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::W(2), .. }));
}


