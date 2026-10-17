/// CCMN (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ccmn_imm.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ccmn_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // CCMN (immediate): if ConditionHolds(cond) then
    //   operand1 = Xn; operand2 = ZeroExtend(imm5)
    //   (-, flags) = AddWithCarry(operand1, operand2, '0)
    //   PSTATE.[N,Z,C,V] = flags
    // end

    let condition_holds = Expr::ConditionHolds {
        condition: semantic_ir::primitives::Condition::Eq,
    };

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let imm5 = 0x1F; // 5-bit immediate
    let imm5_bits = BitWidth::new(5).unwrap();
    let operand2 = Expr::ZeroExtend {
        value: Box::new(Expr::Const {
            value: imm5,
            ty: imm5_bits,
        }),
        to: BitWidth::W64,
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(false)),
        width: BitWidth::W64,
    };

    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let mut semantics = vec![];

    for (flag, bit) in [(Flag::N, 3), (Flag::Z, 2), (Flag::C, 1), (Flag::V, 0)] {
        semantics.push(Stmt::WriteFlag {
            flag,
            value: Expr::BitTest {
                value: Box::new(nzcv.clone()),
                bit,
            },
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: "CCMN".to_string(),
            encoding: Some("Conditional compare negative (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ccmn_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ccmn_imm_validates() {
    let instruction = ccmn_imm_instruction();

    validate_instruction(&instruction).expect("CCMN (immediate) should validate");
}

#[test]
fn ccmn_imm_uses_zero_extend_for_immediate() {
    let instruction = ccmn_imm_instruction();

    assert_eq!(instruction.semantics.len(), 4);

    let Stmt::WriteFlag { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteFlag");
    };

    let Expr::BitTest { value: nzcv, .. } = value else {
        panic!("expected BitTest");
    };

    let Expr::TupleElement { value: add_with_carry, index: 1, .. } = nzcv.as_ref() else {
        panic!("expected TupleElement");
    };

    let Expr::AddWithCarry { right, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    let Expr::ZeroExtend { value: ze_value, to } = right.as_ref() else {
        panic!("expected ZeroExtend");
    };

    assert_eq!(*to, BitWidth::W64);
    let imm5_bits = BitWidth::new(5).unwrap();
    assert!(matches!(ze_value.as_ref(), Expr::Const { ty, .. } if *ty == imm5_bits));
}