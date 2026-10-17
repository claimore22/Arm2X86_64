/// LDTADD, LDTADDA, LDTADDAL, LDTADDL (Atomic add unprivileged unprivileged)
/// Stanford source: arm64_md_clean/ldtadd.md
/// semantics are representable in the IR.
///
/// Unprivileged atomic (memory effects behave as if EL0). Atomically loads from memory,, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Add, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldtadd_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Add,
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
            encoding: Some("Atomic add unprivileged".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldtadd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldtadd_validates() {
    validate_instruction(&ldtadd_instruction("LDTADD", MemSize::Bits64, MemOrdering::Relaxed))
        .expect("LDTADD should validate");
}

#[test]
fn ldtadda_validates_with_acquire() {
    validate_instruction(&ldtadd_instruction("LDTADDA", MemSize::Bits64, MemOrdering::Acquire))
        .expect("LDTADDA should validate");
}

#[test]
fn ldtaddal_validates_with_acquire_release() {
    validate_instruction(&ldtadd_instruction(
        "LDTADDAL",
        MemSize::Bits64,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDTADDAL should validate");
}

#[test]
fn ldtaddl_validates_with_release() {
    validate_instruction(&ldtadd_instruction("LDTADDL", MemSize::Bits64, MemOrdering::Release))
        .expect("LDTADDL should validate");
}



#[test]
fn ldtadd_requires_feat_lse() {
    assert!(ldtadd_instruction("LDTADD", MemSize::Bits64, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn ldtadd_uses_atomic_rmw_op() {
    let instruction = ldtadd_instruction("LDTADDAL", MemSize::Bits64, MemOrdering::AcquireRelease);

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

    assert_eq!(*op, AtomicRMWOp::Add);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}

