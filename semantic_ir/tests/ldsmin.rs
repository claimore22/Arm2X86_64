/// LDSMIN, LDSMINA, LDSMINAL, LDSMINL (Atomic signed minimum on word or doubleword)
/// Stanford source: arm64_md_clean/ldsmin.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, applies signed minimum with Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Smin, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldsmin_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Smin,
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
            encoding: Some("Atomic signed minimum".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldsmin.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldsmin_validates() {
    validate_instruction(&ldsmin_instruction("LDSMINB", MemSize::Bits64, MemOrdering::Relaxed))
        .expect("LDSMIN should validate");
}

#[test]
fn ldsmina_validates_with_acquire() {
    validate_instruction(&ldsmin_instruction("LDSMINA", MemSize::Bits64, MemOrdering::Acquire))
        .expect("LDSMIN should validate");
}

#[test]
fn ldsminal_validates_with_acquire_release() {
    validate_instruction(&ldsmin_instruction(
        "LDSMINAL",
        MemSize::Bits64,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDSMIN should validate");
}

#[test]
fn ldsminl_validates_with_release() {
    validate_instruction(&ldsmin_instruction("LDSMINL", MemSize::Bits64, MemOrdering::Release))
        .expect("LDSMIN should validate");
}

#[test]
fn ldsmin_subword_variants_validate() {
    validate_instruction(&ldsmin_instruction("LDSMINAB", MemSize::Bits8, MemOrdering::Acquire))
        .expect("LDSMIN B-variant should validate");
    validate_instruction(&ldsmin_instruction("LDSMINALH", MemSize::Bits16, MemOrdering::AcquireRelease))
        .expect("LDSMIN H-variant should validate");
}
#[test]
fn ldsmin_requires_feat_lse() {
    assert!(ldsmin_instruction("LDSMINB", MemSize::Bits64, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn ldsmin_uses_atomic_rmw_op() {
    let instruction = ldsmin_instruction("LDSMINAL", MemSize::Bits64, MemOrdering::AcquireRelease);

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

    assert_eq!(*op, AtomicRMWOp::Smin);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}


