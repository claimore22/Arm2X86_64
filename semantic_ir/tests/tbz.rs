/// TBZ (Test Bit and Branch if Zero) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/tbz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn tbz_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // operand = Xt
    let operand = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // bit_pos = 5 (example)
    let bit_pos = 5;

    // offset = SignExtend(imm14::'00')
    let offset = Expr::Const {
        value: 0x1000,
        ty: BitWidth::W64,
    };

    // TBZ: if operand[bit_pos] == '0' then BranchTo(PC + offset)
    // Extract the bit at position bit_pos
    let bit_test = Expr::BitTest {
        value: Box::new(operand),
        bit: bit_pos,
    };

    // Condition is (bit == 0) which is equivalent to !bit_test
    // But BitTest returns Bool, so we need to negate it
    let condition = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(bit_test),
        ty: ValueType::Bool,
    };

    let target = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: semantic_ir::SpecialReg::Pc,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::BranchConditional {
        condition,
        target,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "TBZ".to_string(),
            encoding: Some("Test bit and branch if zero".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "tbz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn tbz_validates() {
    let instruction = tbz_instruction();

    validate_instruction(&instruction).expect("TBZ should validate");
}

#[test]
fn tbz_uses_bit_test_and_branch() {
    let instruction = tbz_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &instruction.semantics[0] else {
        panic!("expected BranchConditional");
    };

    // Condition should be NOT(BitTest(operand, bit_pos))
    let Expr::Unary { op, value, .. } = condition else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, semantic_ir::UnaryOp::Not);

    let Expr::BitTest { value, bit } = value.as_ref() else {
        panic!("expected BitTest");
    };

    assert_eq!(*bit, 5);
    assert!(matches!(value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    let Expr::Binary { op: op2, left: left2, right: right2, .. } = target else {
        panic!("expected Binary Add for target");
    };

    assert_eq!(*op2, semantic_ir::BinOp::Add);
    assert!(matches!(left2.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc, .. }));
    assert!(matches!(right2.as_ref(), Expr::Const { .. }));
}