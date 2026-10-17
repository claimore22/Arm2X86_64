/// SWP, SWPA, SWPAL, SWPL (Swap Word or Doubleword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/swp.md
/// Atomic swap. Requires FEAT_LSE.
use semantic_ir::{
    AtomicRMWOp, BitWidth, Expr, Instruction, InstructionId, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn swp_instruction(mnemonic: &str, size: MemSize, ordering: MemOrdering) -> Instruction {
    let width = size.width();

    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let data = Expr::ReadReg {
        reg: Reg::X(2),
        width,
    };

    let old = Expr::MemAtomicRMW {
        op: AtomicRMWOp::Swp,
        address: Box::new(address),
        value: Box::new(data),
        size,
        ordering,
    };

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
            encoding: Some("Swap Word or Doubleword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "swp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn swp_validates() {
    validate_instruction(&swp_instruction("SWP", MemSize::Bits64, MemOrdering::Relaxed))
        .expect("SWP should validate");
}

#[test]
fn swpa_validates() {
    validate_instruction(&swp_instruction("SWPA", MemSize::Bits64, MemOrdering::Acquire))
        .expect("SWPA should validate");
}

#[test]
fn swpal_validates() {
    validate_instruction(&swp_instruction(
        "SWPAL",
        MemSize::Bits64,
        MemOrdering::AcquireRelease,
    ))
    .expect("SWPAL should validate");
}

#[test]
fn swpl_validates() {
    validate_instruction(&swp_instruction("SWPL", MemSize::Bits64, MemOrdering::Release))
        .expect("SWPL should validate");
}
