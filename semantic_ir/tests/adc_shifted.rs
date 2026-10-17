/// ADC (Add with Carry) shifted register semantics are representable in the IR.
/// Stanford source: arm64_md_clean/adc.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType,
    Width, validate_instruction,
};

fn adc_shifted_instruction() -> Instruction {
    // This test instantiates the instruction at datasize = 64.
    //
    // The Stanford pseudocode uses symbolic `datasize`, but this particular
    // IR fixture is a concrete 64-bit instance.
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::ReadFlag {
            flag: Flag::C,
        }),
        width: BitWidth::W64,
    };

    // Stanford:
    //
    // (result, -) = AddWithCarry{datasize}(operand1, operand2, PSTATE.C);
    //
    // Tuple element 0 is the arithmetic result.
    // Tuple element 1 (NZCV) is explicitly discarded.
    let result = Expr::TupleElement {
        value: Box::new(add_with_carry.clone()),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADC".to_string(),
            encoding: Some("Add with carry".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adc_shifted_register_validates() {
    let instruction = adc_shifted_instruction();

    validate_instruction(&instruction).expect("ADC shifted register should validate");
}

#[test]
fn adc_shifted_register_writes_result_only() {
    let instruction = adc_shifted_instruction();

    // ADC writes only the destination register, no flags
    assert_eq!(instruction.semantics.len(), 1);

    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn adc_shifted_register_uses_readflag_carry() {
    let instruction = adc_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::TupleElement {
        value: add_with_carry,
        index: 0,
..
    } = value
    else {
        panic!("expected AddWithCarry tuple result");
    };

    let Expr::AddWithCarry { carry, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert!(matches!(carry.as_ref(), Expr::ReadFlag { flag: Flag::C }));
}

#[test]
fn adc_shifted_register_discards_nzcv() {
    let instruction = adc_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::TupleElement {
        value: add_with_carry,
        index: 0,
        ty,
..
    } = value
    else {
        panic!("expected AddWithCarry tuple result with index 0");
    };

    // Verify we're selecting the result (index 0), not NZCV (index 1)
    assert_eq!(*ty, ValueType::Bits(Width::Fixed(BitWidth::W64)));

    // Verify the AddWithCarry exists but we don't use its NZCV output
    let Expr::AddWithCarry { width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
}
