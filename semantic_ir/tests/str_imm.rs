/// STR (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/str_imm_gen.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn str_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let offset = Expr::Const {
        value: 0x80,
        ty: BitWidth::W64,
    };

    // STR (immediate): address = base + offset; Mem[address] = Xt
    let base = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Data to store = Xt
    let data = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let semantics = vec![Stmt::MemWrite {
        address,
        value: data,
        size: semantic_ir::MemSize::Bits64,
        access: semantic_ir::MemAccess::Write,
        ordering: semantic_ir::MemOrdering::Relaxed,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "STR".to_string(),
            encoding: Some("Store register (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "str_imm_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn str_imm_validates() {
    let instruction = str_imm_instruction();

    validate_instruction(&instruction).expect("STR (immediate) should validate");
}

#[test]
fn str_imm_uses_mem_write_with_base_plus_offset() {
    let instruction = str_imm_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::MemWrite { address, value, size, .. } = &instruction.semantics[0] else {
        panic!("expected MemWrite");
    };

    assert_eq!(*size, semantic_ir::MemSize::Bits64);

    // Address should be base + offset
    let Expr::Binary { op, left, right, .. } = address else {
        panic!("expected Binary Add for address");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));

    // Value should be Xt
    assert!(matches!(value, Expr::ReadReg { reg: Reg::X(1), .. }));
}