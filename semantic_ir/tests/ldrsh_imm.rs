/// LDRSH (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldrsh_imm.md
///
/// 
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldrsh_imm_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();

        // address = X(n) + imm (scaled/unscaled immediate)
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 16, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };
    let data = Expr::MemRead { address: Box::new(address), size };

    let v = Expr::SignExtend { value: Box::new(data), to: BitWidth::W64 };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: v }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load register signed halfword (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldrsh_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldrsh_imm_validates() {
    validate_instruction(&ldrsh_imm_instruction("LDRSH", MemSize::Bits16))
        .expect("LDRSH should validate");
}

#[test]
fn ldrsh_imm_reads_memory() {
    let instruction = ldrsh_imm_instruction("LDRSH", MemSize::Bits16);

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
fn ldrsh_imm_writes_destination() {
    let instruction = ldrsh_imm_instruction("LDRSH", MemSize::Bits16);
    assert!(instruction
        .semantics
        .iter()
        .any(|s| matches!(s, Stmt::WriteReg { .. })));
}
