/// TBNZ (Test Bit and Branch if Nonzero) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/tbnz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn tbnz_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let bit_pos = 5;

    let offset = Expr::Const {
        value: 0x1000,
        ty: BitWidth::W64,
    };

    // TBNZ: if operand[bit_pos] == '1' then BranchTo(PC + offset)
    // This is just BitTest directly (no Not needed)
    let condition = Expr::BitTest {
        value: Box::new(operand),
        bit: bit_pos,
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
            mnemonic: "TBNZ".to_string(),
            encoding: Some("Test bit and branch if nonzero".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "tbnz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn tbnz_validates() {
    let instruction = tbnz_instruction();

    validate_instruction(&instruction).expect("TBNZ should validate");
}

#[test]
fn tbnz_uses_bit_test_and_branch() {
    let instruction = tbnz_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &instruction.semantics[0] else {
        panic!("expected BranchConditional");
    };

    // Condition should be BitTest(operand, bit_pos)
    let Expr::BitTest { value, bit } = condition else {
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