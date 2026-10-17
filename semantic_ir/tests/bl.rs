/// BL (Branch with Link) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bl.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn bl_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // offset = SignExtend(imm26::'00')
    let offset = Expr::Const {
        value: 0x100000, // Example 26-bit offset << 2
        ty: BitWidth::W64,
    };

    // BL: X30 = PC + 4; BranchTo(PC + offset, DIRCALL)
    // First: X30 = PC + 4
    let pc_plus_4 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc,
            width: BitWidth::W64,
        }),
        right: Box::new(Expr::Const {
            value: 4,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Then: BranchTo(PC + offset, DIRCALL)
    let target = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(30), // X30 is the link register
            value: pc_plus_4,
        },
        Stmt::Branch { target },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "BL".to_string(),
            encoding: Some("Branch with link".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bl_validates() {
    let instruction = bl_instruction();

    validate_instruction(&instruction).expect("BL should validate");
}

#[test]
fn bl_writes_link_register_and_branches() {
    let instruction = bl_instruction();

    assert_eq!(instruction.semantics.len(), 2);

    // First: Write X30 = PC + 4
    let Stmt::WriteReg { reg, value } = &instruction.semantics[0] else {
        panic!("expected WriteReg for X30");
    };

    assert_eq!(*reg, Reg::X(30));

    let Expr::Binary { op, left, right, .. } = value else {
        panic!("expected Binary Add for PC + 4");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, .. }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 4, .. }));

    // Second: Branch to target
    let Stmt::Branch { target } = &instruction.semantics[1] else {
        panic!("expected Branch");
    };

    let Expr::Binary { op, left, right, .. } = target else {
        panic!("expected Binary Add for target");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, .. }));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));
}