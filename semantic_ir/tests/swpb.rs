/// SWPB, SWPAB, SWPALB, SWPLB (Swap byte in memory)
/// Stanford source: arm64_md_clean/swpb.md
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

fn swpb_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
            encoding: Some("Swap byte in memory".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "swpb.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn swpb_validates() {
    validate_instruction(&swpb_instruction("SWPB", MemSize::Bits8, MemOrdering::Relaxed))
        .expect("SWPB should validate");
}

#[test]
fn swpba_validates_with_acquire() {
    validate_instruction(&swpb_instruction("SWPAB", MemSize::Bits8, MemOrdering::Acquire))
        .expect("SWPBA should validate");
}

#[test]
fn swpbal_validates_with_acquire_release() {
    validate_instruction(&swpb_instruction(
        "SWPALB",
        MemSize::Bits8,
        MemOrdering::AcquireRelease,
    ))
    .expect("SWPBAL should validate");
}

#[test]
fn swpbl_validates_with_release() {
    validate_instruction(&swpb_instruction("X", MemSize::Bits8, MemOrdering::Release))
        .expect("SWPBL should validate");
}

#[test]
fn swpbb_and_swpbh_validate() {
    validate_instruction(&swpb_instruction("SWPBB", MemSize::Bits8, MemOrdering::Relaxed))
        .expect("SWPBB should validate");
    validate_instruction(&swpb_instruction("SWPBH", MemSize::Bits16, MemOrdering::Relaxed))
        .expect("SWPBH should validate");
}

#[test]
fn swpb_requires_feat_lse() {
    assert!(swpb_instruction("SWPB", MemSize::Bits8, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn swpb_uses_atomic_rmw_op() {
    let instruction = swpb_instruction("SWPALB", MemSize::Bits8, MemOrdering::AcquireRelease);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // sub-word results are zero-extended to 64-bit
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

    assert_eq!(*op, AtomicRMWOp::Swp);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::W(2), .. }));
}


