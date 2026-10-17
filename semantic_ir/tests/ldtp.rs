/// LDTP, LDTNP (Load unprivileged pair of registers) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldtp_gen.md
///
/// Pair of loads from X(n)+imm7, modeled as two MemRead+WriteReg.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldtp_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();
    let dbytes = (size.width().bits() / 8) as u128;

    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let addr2 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base.clone()),
        right: Box::new(Expr::Const { value: dbytes, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let data0 = Expr::MemRead { address: Box::new(base), size };
    let data1 = Expr::MemRead { address: Box::new(addr2), size };
    let v0 = if width != BitWidth::W64 {
        Expr::ZeroExtend { value: Box::new(data0), to: BitWidth::W64 }
    } else { data0 };
    let v1 = if width != BitWidth::W64 {
        Expr::ZeroExtend { value: Box::new(data1), to: BitWidth::W64 }
    } else { data1 };
    let semantics = vec![
        Stmt::WriteReg { reg: Reg::X(0), value: v0 },
        Stmt::WriteReg { reg: Reg::X(2), value: v1 },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load pair of registers".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldtp_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldtp_validates() {
    validate_instruction(&ldtp_instruction("LDTP", MemSize::Bits64))
        .expect("LDNP should validate");
}

#[test]
fn ldtp_reads_memory() {
    let instruction = ldtp_instruction("LDTP", MemSize::Bits64);

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
fn ldtp_writes_destination() {
    let instruction = ldtp_instruction("LDTP", MemSize::Bits64);
    assert!(instruction
        .semantics
        .iter()
        .any(|s| matches!(s, Stmt::WriteReg { .. })));
}

