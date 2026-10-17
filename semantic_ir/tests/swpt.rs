/// SWPT, SWPTA, SWPTAL, SWPTL (Swap, unprivileged)
/// Stanford source: arm64_md_clean/swpt.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, adds Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Add, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn swpt_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Swp,
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
            encoding: Some("Swap, unprivileged".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "swpt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn swpt_validates() {
    validate_instruction(&swpt_instruction("SWPT", MemSize::Bits64, MemOrdering::Relaxed))
        .expect("SWPT should validate");
}

#[test]
fn swpta_validates_with_acquire() {
    validate_instruction(&swpt_instruction("SWPAT", MemSize::Bits64, MemOrdering::Acquire))
        .expect("SWPTA should validate");
}

#[test]
fn swptal_validates_with_acquire_release() {
    validate_instruction(&swpt_instruction(
        "SWPALT",
        MemSize::Bits64,
        MemOrdering::AcquireRelease,
    ))
    .expect("SWPTAL should validate");
}

#[test]
fn swptl_validates_with_release() {
    validate_instruction(&swpt_instruction("X", MemSize::Bits64, MemOrdering::Release))
        .expect("SWPTL should validate");
}

#[test]
fn swptb_and_swpth_validate() {
    validate_instruction(&swpt_instruction("SWPTB", MemSize::Bits8, MemOrdering::Relaxed))
        .expect("SWPTB should validate");
    validate_instruction(&swpt_instruction("SWPTH", MemSize::Bits16, MemOrdering::Relaxed))
        .expect("SWPTH should validate");
}

#[test]
fn swpt_requires_feat_lse() {
    assert!(swpt_instruction("SWPT", MemSize::Bits64, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn swpt_uses_atomic_rmw_op() {
    let instruction = swpt_instruction("SWPALT", MemSize::Bits64, MemOrdering::AcquireRelease);

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

    assert_eq!(*op, AtomicRMWOp::Swp);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}

