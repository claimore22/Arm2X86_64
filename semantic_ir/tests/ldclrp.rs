/// LDCLRP, LDCLRPA, LDCLRPAL, LDCLRPL (Atomic bit clear on quadword) semantics
/// Stanford source: arm64_md_clean/ldclrp.md
/// are representable in the IR.
///
/// Atomically loads a 128-bit value from memory, clears the bits held in the
/// register pair X(s+1):X(s), stores the result back, and returns the original
/// 128-bit memory value in the register pair X(t+1):X(t). Modeled with
/// `Expr::MemAtomicRMW { op: Bic, size: Bits128, .. }`, with the 128-bit
/// operand/result composed via Concat/Extract of register pairs.
/// Requires FEAT_LSE128.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn ldclrp_instruction(mnemonic: &str, ordering: MemOrdering) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    // 128-bit operand is a register pair (X(s+1):X(s))
    let value = Expr::Concat {
        high: Box::new(Expr::ReadReg {
            reg: Reg::X(3),
            width: BitWidth::W64,
        }),
        low: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        width: BitWidth::W128,
    };

    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Bic,
        address: Box::new(address),
        value: Box::new(value),
        size: MemSize::Bits128,
        ordering,
    };

    // original value returned in register pair X(t+1):X(t)
    let lo = Expr::Extract {
        value: Box::new(old.clone()),
        lsb: 0,
        width: BitWidth::W64,
    };
    let hi = Expr::Extract {
        value: Box::new(old),
        lsb: 64,
        width: BitWidth::W64,
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(4),
            value: lo,
        },
        Stmt::WriteReg {
            reg: Reg::X(5),
            value: hi,
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Atomic bit clear on quadword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE128".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldclrp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldclrp_validates() {
    validate_instruction(&ldclrp_instruction("LDCLRP", MemOrdering::Relaxed))
        .expect("LDCLRP should validate");
}

#[test]
fn ldclrpa_validates_with_acquire() {
    validate_instruction(&ldclrp_instruction("LDCLRPA", MemOrdering::Acquire))
        .expect("LDCLRPA should validate");
}

#[test]
fn ldclrpal_validates_with_acquire_release() {
    validate_instruction(&ldclrp_instruction("LDCLRPAL", MemOrdering::AcquireRelease))
        .expect("LDCLRPAL should validate");
}

#[test]
fn ldclrpl_validates_with_release() {
    validate_instruction(&ldclrp_instruction("LDCLRPL", MemOrdering::Release))
        .expect("LDCLRPL should validate");
}

#[test]
fn ldclrp_requires_feat_lse128() {
    assert!(ldclrp_instruction("LDCLRP", MemOrdering::Relaxed)
        .required_features
        .contains(&"FEAT_LSE128".to_string()));
}

#[test]
fn ldclrp_uses_atomic_rmw_bic_128() {
    let instruction = ldclrp_instruction("LDCLRP", MemOrdering::Relaxed);

    let Stmt::WriteReg { value: lo, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Extract { value, width, .. } = lo else {
        panic!("expected Extract");
    };
    assert_eq!(*width, BitWidth::W64);

    let Expr::MemAtomicRMW { op, size, .. } = value.as_ref() else {
        panic!("expected MemAtomicRMW");
    };

    assert_eq!(*op, AtomicRMWOp::Bic);
    assert_eq!(*size, MemSize::Bits128);
}

#[test]
fn ldclrp_returns_register_pair() {
    let instruction = ldclrp_instruction("LDCLRP", MemOrdering::Relaxed);

    assert_eq!(instruction.semantics.len(), 2);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(4), .. }
    ));
    assert!(matches!(
        instruction.semantics[1],
        Stmt::WriteReg { reg: Reg::X(5), .. }
    ));
}
