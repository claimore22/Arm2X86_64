/// LDRSW (literal) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldrsw_lit.md
///
/// PC-relative load.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldrsw_lit_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();

        // address = PC + imm (literal load)
    let base = Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, width: BitWidth::W64 };
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 64, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };
    let data = Expr::MemRead { address: Box::new(address), size };

    let v = Expr::SignExtend { value: Box::new(data), to: BitWidth::W64 };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: v }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load register signed word (literal)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldrsw_lit.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldrsw_lit_validates() {
    validate_instruction(&ldrsw_lit_instruction("LDRSW", MemSize::Bits32))
        .expect("LDRSW should validate");
}

#[test]
fn ldrsw_lit_reads_memory() {
    let instruction = ldrsw_lit_instruction("LDRSW", MemSize::Bits32);

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
fn ldrsw_lit_writes_destination() {
    let instruction = ldrsw_lit_instruction("LDRSW", MemSize::Bits32);
    assert!(instruction
        .semantics
        .iter()
        .any(|s| matches!(s, Stmt::WriteReg { .. })));
}
