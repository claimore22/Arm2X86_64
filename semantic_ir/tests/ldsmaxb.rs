/// LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB (Atomic signed maximum on byte)
/// Stanford source: arm64_md_clean/ldsmaxb.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, applies signed maximum with Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Smax, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldsmaxb_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Smax,
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
            encoding: Some("Atomic signed maximum".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldsmaxb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldsmaxb_validates() {
    validate_instruction(&ldsmaxb_instruction("LDSMAXBB", MemSize::Bits8, MemOrdering::Relaxed))
        .expect("LDSMAXB should validate");
}

#[test]
fn ldsmaxba_validates_with_acquire() {
    validate_instruction(&ldsmaxb_instruction("LDSMAXAB", MemSize::Bits8, MemOrdering::Acquire))
        .expect("LDSMAXB should validate");
}

#[test]
fn ldsmaxbal_validates_with_acquire_release() {
    validate_instruction(&ldsmaxb_instruction(
        "LDSMAXALB",
        MemSize::Bits8,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDSMAXB should validate");
}

#[test]
fn ldsmaxbl_validates_with_release() {
    validate_instruction(&ldsmaxb_instruction("LDSMAXLB", MemSize::Bits8, MemOrdering::Release))
        .expect("LDSMAXB should validate");
}

#[test]
fn ldsmaxb_subword_variants_validate() {
    validate_instruction(&ldsmaxb_instruction("LDSMAXBAB", MemSize::Bits8, MemOrdering::Acquire))
        .expect("LDSMAXB B-variant should validate");
    validate_instruction(&ldsmaxb_instruction("LDSMAXBALH", MemSize::Bits16, MemOrdering::AcquireRelease))
        .expect("LDSMAXB H-variant should validate");
}
#[test]
fn ldsmaxb_requires_feat_lse() {
    assert!(ldsmaxb_instruction("LDSMAXBB", MemSize::Bits8, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn ldsmaxb_uses_atomic_rmw_op() {
    let instruction = ldsmaxb_instruction("LDSMAXALB", MemSize::Bits8, MemOrdering::AcquireRelease);

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

    assert_eq!(*op, AtomicRMWOp::Smax);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::W(2), .. }));
}


