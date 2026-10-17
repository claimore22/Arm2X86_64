/// LDR (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldr_imm_gen.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn ldr_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // offset = LSL(ZeroExtend(imm12), scale)
    // For example: imm12 = 0x10, scale = 3 (for 64-bit)
    let offset = Expr::Const {
        value: 0x80, // 0x10 << 3
        ty: BitWidth::W64,
    };

    // LDR (immediate): address = base + offset; Xt = Mem[address]
    // base = Xn or SP
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

    let mem_read = Expr::MemRead {
        address: Box::new(address),
        size: semantic_ir::MemSize::Bits64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: mem_read,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDR".to_string(),
            encoding: Some("Load register (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldr_imm_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldr_imm_validates() {
    let instruction = ldr_imm_instruction();

    validate_instruction(&instruction).expect("LDR (immediate) should validate");
}

#[test]
fn ldr_imm_uses_mem_read_with_base_plus_offset() {
    let instruction = ldr_imm_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::MemRead { address, size } = value else {
        panic!("expected MemRead");
    };

    assert_eq!(*size, semantic_ir::MemSize::Bits64);

    // Address should be base + offset
    let Expr::Binary { op, left, right, .. } = address.as_ref() else {
        panic!("expected Binary Add for address");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));
}