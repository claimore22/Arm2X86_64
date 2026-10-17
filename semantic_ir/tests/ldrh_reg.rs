/// LDRH (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldrh_reg.md
///
/// 
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldrh_reg_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();

        // address = X(n) + LSL(ZeroExtend(X(m)), shift)
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let index = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        amount: Box::new(Expr::Const { value: 3, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(index),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };
    let data = Expr::MemRead { address: Box::new(address), size };

    let v = if width != BitWidth::W64 {
        Expr::ZeroExtend { value: Box::new(data), to: BitWidth::W64 }
    } else { data };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: v }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load register halfword (register offset)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldrh_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldrh_reg_validates() {
    validate_instruction(&ldrh_reg_instruction("LDRH", MemSize::Bits16))
        .expect("LDRH should validate");
}

#[test]
fn ldrh_reg_reads_memory() {
    let instruction = ldrh_reg_instruction("LDRH", MemSize::Bits16);

    fn has_mem_read(expr: &Expr) -> bool {
        match expr {
            Expr::MemRead { .. } => true,
            Expr::ZeroExtend { value, .. } | Expr::SignExtend { value, .. } => has_mem_read(value),
            Expr::Binary { left, right, .. } => has_mem_read(left) || has_mem_read(right),
            Expr::ExtendReg { register, shift, .. } => has_mem_read(shift),
            _ => false,
        }
    }

    let found = instruction.semantics.iter().any(|s| match s {
        Stmt::WriteReg { value, .. } => has_mem_read(value),
        _ => false,
    });
    assert!(found, "expected a memory read");
}

#[test]
fn ldrh_reg_writes_destination() {
    let instruction = ldrh_reg_instruction("LDRH", MemSize::Bits16);
    assert!(instruction
        .semantics
        .iter()
        .any(|s| matches!(s, Stmt::WriteReg { .. })));
}
