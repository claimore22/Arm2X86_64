/// STTXR (Store unprivileged exclusive register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sttxr.md
///
/// Exclusive store: checks exclusive monitor, stores, and writes a 0/1 status
/// to Ws. Modeled as exclusive-monitor placeholder Let + MemWrite + status
/// WriteReg. 
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn sttxr_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let mut semantics = vec![Stmt::Let {
        name: "check_exclusive_monitor".to_string(),
        value: Expr::Const { value: 1, ty: BitWidth::W8 },
    }];
    semantics.push(Stmt::MemWrite {
        address,
        value: Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 },
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    });
    // status register: 0 (success)
    semantics.push(Stmt::WriteReg {
        reg: Reg::W(3),
        value: Expr::Const { value: 0, ty: BitWidth::W32 },
    });

    Instruction {
        id: InstructionId { mnemonic: "STTXR".to_string(), encoding: Some("Store unprivileged exclusive register".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC3".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sttxr.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sttxr_validates() {
    validate_instruction(&sttxr_instruction()).expect("should validate");
}

#[test]
fn sttxr_writes_status_zero() {
    let instruction = sttxr_instruction();
    let last = instruction.semantics.last().expect("has stmts");
    assert!(matches!(
        last,
        Stmt::WriteReg {
            value: Expr::Const { value: 0, .. },
..
        }
    ));
}
