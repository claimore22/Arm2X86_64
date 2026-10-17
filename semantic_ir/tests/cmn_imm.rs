/// CMN (immediate) is an alias of ADDS (immediate) that discards the result.
/// Stanford source: arm64_md_clean/cmn_adds_addsub_imm.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn cmn_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let n_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };

    let operand1 = Expr::Select {
        condition: Box::new(n_is_sp),
        when_true: Box::new(Expr::ReadSP {
            width: BitWidth::W64,
        }),
        when_false: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let imm_bits = BitWidth::new(24).unwrap();
    let operand2 = Expr::ZeroExtend {
        value: Box::new(Expr::Const {
            value: 0x123456,
            ty: imm_bits,
        }),
        to: BitWidth::W64,
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(false)),
        width: BitWidth::W64,
    };

    // CMN discards the result (tuple element 0), only extracts flags (tuple element 1)
    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let mut semantics = Vec::new();

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
            mnemonic: "CMN".to_string(),
            encoding: Some("Compare negative (immediate, alias of ADDS)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmn_adds_addsub_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmn_imm_validates() {
    let instruction = cmn_imm_instruction();

    validate_instruction(&instruction).expect("CMN immediate should validate");
}

#[test]
fn cmn_imm_writes_only_flags() {
    let instruction = cmn_imm_instruction();

    // CMN writes only 4 flags, no result register
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
..
            } if *flag == expected_flag
        ));
    }
}

#[test]
fn cmn_imm_uses_zero_extend_for_immediate() {
    let instruction = cmn_imm_instruction();

    // The flags are extracted from AddWithCarry result (tuple element 1)
    let Stmt::WriteFlag { value, .. } = &instruction.semantics[0] else {
        panic!("expected flag write");
    };

    let Expr::BitTest { value: nzcv, bit: _ } = value else {
        panic!("expected BitTest for flag");
    };

    let Expr::TupleElement {
        value: add_with_carry,
        index: 1,
..
    } = nzcv.as_ref()
    else {
        panic!("expected TupleElement index 1 for flags");
    };

    let Expr::AddWithCarry { right, carry, width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
    assert!(matches!(carry.as_ref(), Expr::Bool(false)));

    let Expr::ZeroExtend { value: imm, .. } = right.as_ref() else {
        panic!("expected ZeroExtend for immediate");
    };

    let Expr::Const { value, ty } = imm.as_ref() else {
        panic!("expected Const for immediate");
    };

    assert_eq!(*value, 0x123456);
    // The immediate is stored as 24-bit but zero-extended to 64-bit
    // The immediate is stored as 24-bit before zero extension
    assert_eq!(*ty, BitWidth::new(24).unwrap());
}
