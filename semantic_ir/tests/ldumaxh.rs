/// LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH (Atomic unsigned maximum on halfword)
/// Stanford source: arm64_md_clean/ldumaxh.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, applies unsigned maximum with Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Umax, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldumaxh_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Umax,
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
            encoding: Some("Atomic unsigned maximum".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldumaxh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldumaxh_validates() {
    validate_instruction(&ldumaxh_instruction("LDUMAXHB", MemSize::Bits16, MemOrdering::Relaxed))
        .expect("LDUMAXH should validate");
}

#[test]
fn ldumaxha_validates_with_acquire() {
    validate_instruction(&ldumaxh_instruction("LDUMAXAH", MemSize::Bits16, MemOrdering::Acquire))
        .expect("LDUMAXH should validate");
}

#[test]
fn ldumaxhal_validates_with_acquire_release() {
    validate_instruction(&ldumaxh_instruction(
        "LDUMAXALH",
        MemSize::Bits16,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDUMAXH should validate");
}

#[test]
fn ldumaxhl_validates_with_release() {
    validate_instruction(&ldumaxh_instruction("LDUMAXLH", MemSize::Bits16, MemOrdering::Release))
        .expect("LDUMAXH should validate");
}

#[test]
fn ldumaxh_subword_variants_validate() {
    validate_instruction(&ldumaxh_instruction("LDUMAXHAB", MemSize::Bits8, MemOrdering::Acquire))
        .expect("LDUMAXH B-variant should validate");
    validate_instruction(&ldumaxh_instruction("LDUMAXHALH", MemSize::Bits16, MemOrdering::AcquireRelease))
        .expect("LDUMAXH H-variant should validate");
}
#[test]
fn ldumaxh_requires_feat_lse() {
    assert!(ldumaxh_instruction("LDUMAXHB", MemSize::Bits16, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn ldumaxh_uses_atomic_rmw_op() {
    let instruction = ldumaxh_instruction("LDUMAXALH", MemSize::Bits16, MemOrdering::AcquireRelease);

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

    assert_eq!(*op, AtomicRMWOp::Umax);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::W(2), .. }));
}


