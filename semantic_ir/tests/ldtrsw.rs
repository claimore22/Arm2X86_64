/// LDTRSW (Load register (unprivileged)) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldtrsw.md
///
/// Load (unprivileged): address = X(n) + imm9; memory effects behave as if
/// executed at EL0. Signed load sign-extends to 64 bits.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldtrsw_instruction() -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 8, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };

    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits32,
    };

    let value = Expr::SignExtend { value: Box::new(data), to: BitWidth::W64 };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDTRSW".to_string(),
            encoding: Some("LDTRSW (Load register (unprivileged))".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldtrsw.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldtrsw_validates() {
    validate_instruction(&ldtrsw_instruction()).expect("LDTRSW should validate");
}

#[test]
fn ldtrsw_reads_memory() {
    let instruction = ldtrsw_instruction();

    fn has_mem_read(expr: &Expr) -> bool {
        match expr {
            Expr::MemRead { .. } => true,
            Expr::ZeroExtend { value, .. } | Expr::SignExtend { value, .. } => has_mem_read(value),
            _ => false,
        }
    }

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(has_mem_read(value));
}

#[test]
fn ldtrsw_writes_destination() {
    let instruction = ldtrsw_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(0), .. }
    ));
}
