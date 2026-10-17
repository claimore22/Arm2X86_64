/// LDEOR, LDEORA, LDEORAL, LDEORL (Atomic exclusive-OR on word or doubleword)
/// Stanford source: arm64_md_clean/ldeor.md
/// semantics are representable in the IR.
///
/// Atomically loads from memory, applies exclusive-OR with Xs, stores the result back, and returns
/// the original memory value in Xt (when t != 31). Modeled with
/// `Expr::MemAtomicRMW { op: Eor, .. }`. The A/R bits select acquire/release
/// ordering; B/H subfamilies differ only in operand size (8/16/32/64).
/// Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldeor_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
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
        op: AtomicRMWOp::Eor,
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
            encoding: Some("Atomic exclusive-OR".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldeor.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldeor_validates() {
    validate_instruction(&ldeor_instruction("LDEOR", MemSize::Bits64, MemOrdering::Relaxed))
        .expect("LDEOR should validate");
}

#[test]
fn ldeora_validates_with_acquire() {
    validate_instruction(&ldeor_instruction("LDEORA", MemSize::Bits64, MemOrdering::Acquire))
        .expect("LDEOR should validate");
}

#[test]
fn ldeoral_validates_with_acquire_release() {
    validate_instruction(&ldeor_instruction(
        "LDEORAL",
        MemSize::Bits64,
        MemOrdering::AcquireRelease,
    ))
    .expect("LDEOR should validate");
}

#[test]
fn ldeorl_validates_with_release() {
    validate_instruction(&ldeor_instruction("LDEORL", MemSize::Bits64, MemOrdering::Release))
        .expect("LDEOR should validate");
}

#[test]
fn ldeorb_and_ldeorh_validate() {
    validate_instruction(&ldeor_instruction("LDEORB", MemSize::Bits8, MemOrdering::Relaxed))
        .expect("LDEOR should validate");
    validate_instruction(&ldeor_instruction("LDEORH", MemSize::Bits16, MemOrdering::Relaxed))
        .expect("LDEOR should validate");
}

#[test]
fn ldeor_requires_feat_lse() {
    assert!(ldeor_instruction("LDEOR", MemSize::Bits64, MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE".to_string()));
}

#[test]
fn ldeor_uses_atomic_rmw_op() {
    let instruction = ldeor_instruction("LDEORAL", MemSize::Bits64, MemOrdering::AcquireRelease);

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

    assert_eq!(*op, AtomicRMWOp::Eor);
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(newvalue.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}

