/// ST64BV0 (Single-copy atomic 64-byte EL0 store with status result) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/st64bv0.md
///
/// Single-copy atomic 64-byte store (with status in Xs): eight 64-bit doublewords from
/// consecutive registers are stored to a 64-byte aligned address.
/// Requires FEAT_LS64.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn st64bv0_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let mut semantics = Vec::new();
    for i in 0..8u8 {
        let addr = Expr::Binary {
            op: semantic_ir::BinOp::Add,
            left: Box::new(base.clone()),
            right: Box::new(Expr::Const { value: (i as u128) * 8, ty: BitWidth::W64 }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        };
        let value = Expr::ReadReg { reg: Reg::X(2 + i), width: BitWidth::W64 };
        semantics.push(Stmt::MemWrite {
            address: addr,
            value,
            size: MemSize::Bits64,
            access: MemAccess::Write,
            ordering: MemOrdering::Relaxed,
        });
    }
        semantics.push(Stmt::WriteReg { reg: Reg::W(0), value: Expr::Const { value: 0, ty: BitWidth::W32 } });

    Instruction {
        id: InstructionId { mnemonic: "ST64BV0".to_string(), encoding: Some("Atomic 64-byte EL0 store with status".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LS64".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "st64bv0.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn st64bv0_validates() {
    validate_instruction(&st64bv0_instruction()).expect("should validate");
}

#[test]
fn st64bv0_stores_eight_registers() {
    let instruction = st64bv0_instruction();
    let n = instruction
        .semantics
        .iter()
        .filter(|s| matches!(s, Stmt::MemWrite { .. }))
        .count();
    assert_eq!(n, 8);
}
