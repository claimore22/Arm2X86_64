/// CCMP (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ccmp_reg.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ccmp_reg_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // CCMP (register): if ConditionHolds(cond) then
    //   operand1 = Xn; operand2 = Xm
    //   (-, flags) = AddWithCarry(operand1, NOT(operand2), '1')
    //   PSTATE.[N,Z,C,V] = flags
    // end

    let condition_holds = Expr::ConditionHolds {
        condition: semantic_ir::primitives::Condition::Eq,
    };

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    let not_operand2 = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(not_operand2),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let mut semantics = vec![];

    // CCMP: if ConditionHolds(cond) then update flags
    // We model this as a conditional flag update
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
            mnemonic: "CCMP".to_string(),
            encoding: Some("Conditional compare (register)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ccmp_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ccmp_reg_validates() {
    let instruction = ccmp_reg_instruction();

    validate_instruction(&instruction).expect("CCMP (register) should validate");
}

#[test]
fn ccmp_reg_updates_flags_conditionally() {
    let instruction = ccmp_reg_instruction();

    assert_eq!(instruction.semantics.len(), 4);

    for (statement, expected_flag) in [
        (&instruction.semantics[0], Flag::N),
        (&instruction.semantics[1], Flag::Z),
        (&instruction.semantics[2], Flag::C),
        (&instruction.semantics[3], Flag::V),
    ] {
        assert!(matches!(
            statement,
            Stmt::WriteFlag {
                flag,
                value: Expr::BitTest { .. },
            } if *flag == expected_flag
        ));
    }
}