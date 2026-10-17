/// LDR (literal) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldr_lit_gen.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn ldr_lit_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // offset = SignExtend(imm19::'00')
    let offset = Expr::Const {
        value: 0x1000,
        ty: BitWidth::W64,
    };

    // address = PC + offset
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // LDR (literal): Xt = Mem[address]
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
            encoding: Some("Load register (literal)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldr_lit_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldr_lit_validates() {
    let instruction = ldr_lit_instruction();

    validate_instruction(&instruction).expect("LDR (literal) should validate");
}

#[test]
fn ldr_lit_uses_mem_read_with_pc_relative_address() {
    let instruction = ldr_lit_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::MemRead { address, size } = value else {
        panic!("expected MemRead");
    };

    assert_eq!(*size, semantic_ir::MemSize::Bits64);

    // Address should be PC + offset
    let Expr::Binary { op, left, right, .. } = address.as_ref() else {
        panic!("expected Binary Add for address");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, .. }));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));
}