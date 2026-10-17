/// STR (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/str_reg_gen.md
///
/// X(n) + extended/shifted X(m): store of 64-bit value from X(t) to address X(n) + extended/shifted X(m).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, ShiftKind, Stmt, ValueType, Width, validate_instruction,
};

fn str_reg_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    // address = X(n) + LSL(ZeroExtend(X(m)/W(m)), shift)
    let index = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        amount: Box::new(Expr::Const { value: 3, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(index),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::MemWrite {
        address,
        value: Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 },
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    }];

    Instruction {
        id: InstructionId { mnemonic: "STR".to_string(), encoding: Some("X(n) + extended/shifted X(m)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "str_reg_gen.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn str_reg_validates() {
    validate_instruction(&str_reg_instruction()).expect("should validate");
}

#[test]
fn str_reg_writes_memory() {
    let instruction = str_reg_instruction();
    assert!(matches!(instruction.semantics[0], Stmt::MemWrite { .. }));
}
