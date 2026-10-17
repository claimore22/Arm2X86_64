/// ADRP (Form PC-relative address to 4KB page) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/adrp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn adrp_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // imm = SignExtend(immhi::immlo::Zeros{12})
    // In practice, this is a 33-bit immediate shifted left by 12
    let imm = Expr::Const {
        value: 0x100000, // Example: 0x1000 << 12
        ty: BitWidth::W64,
    };

// ADRP: base = PC[63:12]::Zeros{12}; Xd = base + imm
    // This means: PC with bottom 12 bits cleared, then add imm
    let pc = Expr::ReadSpecial {
        reg: semantic_ir::SpecialReg::Pc64,
        width: BitWidth::W64,
    };

    // Clear bottom 12 bits of PC: PC & ~0xFFF
    let mask = Expr::Const {
        value: 0xFFFFFFFFFFFFF000u128,
        ty: BitWidth::W64,
    };

    let pc_aligned = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(pc),
        right: Box::new(mask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(pc_aligned),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADRP".to_string(),
            encoding: Some("Form PC-relative address to 4KB page".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adrp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adrp_validates() {
    let instruction = adrp_instruction();

    validate_instruction(&instruction).expect("ADRP should validate");
}

#[test]
fn adrp_computes_page_aligned_pc_relative_address() {
    let instruction = adrp_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Result = (PC & ~0xFFF) + imm
    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Add");
    };

    assert_eq!(*op1, semantic_ir::BinOp::Add);
    assert!(matches!(right.as_ref(), Expr::Const { .. }));

    // left = PC & ~0xFFF
    let Expr::Binary { op: op2, left: left2, right: right2, .. } = left.as_ref() else {
        panic!("expected Binary And for PC alignment");
    };

    assert_eq!(*op2, semantic_ir::BinOp::And);
    assert!(matches!(left2.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc64, .. }));
    assert!(matches!(right2.as_ref(), Expr::Const { value: 0xFFFFFFFFFFFFF000u128, .. }));
}