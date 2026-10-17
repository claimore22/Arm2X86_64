/// STLXP (Store-release exclusive pair of registers) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stlxp.md
///
/// Exclusive store: checks exclusive monitor, stores, and writes a 0/1 status
/// to Ws. Modeled as exclusive-monitor placeholder Let + MemWrite + status
/// WriteReg. 
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, validate_instruction,
};

fn stlxp_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let mut semantics = vec![Stmt::Let {
        name: "check_exclusive_monitor".to_string(),
        value: Expr::Const { value: 1, ty: BitWidth::W8 },
    }];
    semantics.push(Stmt::MemWrite {
        address: address.clone(),
        value: Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 },
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    });
    semantics.push(Stmt::MemWrite {
        address: Expr::Binary {
            op: semantic_ir::BinOp::Add,
            left: Box::new(address),
            right: Box::new(Expr::Const { value: 8, ty: BitWidth::W64 }),
            ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
        },
        value: Expr::ReadReg { reg: Reg::X(4), width: BitWidth::W64 },
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    });
    // status register: 0 (success)
    semantics.push(Stmt::WriteReg {
        reg: Reg::W(3),
        value: Expr::Const { value: 0, ty: BitWidth::W32 },
    });

    Instruction {
        id: InstructionId { mnemonic: "STLXP".to_string(), encoding: Some("Store-release exclusive pair".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stlxp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stlxp_validates() {
    validate_instruction(&stlxp_instruction()).expect("should validate");
}

#[test]
fn stlxp_writes_status_zero() {
    let instruction = stlxp_instruction();
    let last = instruction.semantics.last().expect("has stmts");
    assert!(matches!(
        last,
        Stmt::WriteReg {
            value: Expr::Const { value: 0, .. },
..
        }
    ));
}
