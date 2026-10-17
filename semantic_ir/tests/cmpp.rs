/// CMPP (Compare with tag) is an alias of SUBPS.
/// Stanford source: arm64_md_clean/cmpp_subps.md
/// It subtracts 56-bit addresses (sign-extended), updates flags, but discards the result.
/// Requires FEAT_MTE.
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, UnaryOp, ValueType,
    Width, validate_instruction,
};

fn cmpp_instruction() -> Instruction {
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

    let m_is_sp = Expr::Binary {
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

    let operand2_raw = Expr::Select {
        condition: Box::new(m_is_sp),
        when_true: Box::new(Expr::ReadSP {
            width: BitWidth::W64,
        }),
        when_false: Box::new(Expr::ReadReg {
            reg: Reg::X(3),
            width: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Stanford: operand1 = SignExtend{64}(operand1[55:0])
    let operand1_sext = Expr::SignExtend {
        value: Box::new(Expr::Extract {
            value: Box::new(operand1),
            lsb: 0,
            width: BitWidth::new(56).unwrap(),
        }),
        to: BitWidth::W64,
    };

    // Stanford: operand2 = NOT(SignExtend{64}(operand2[55:0]))
    let operand2_sext = Expr::SignExtend {
        value: Box::new(Expr::Extract {
            value: Box::new(operand2_raw),
            lsb: 0,
            width: BitWidth::new(56).unwrap(),
        }),
        to: BitWidth::W64,
    };

    let operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(operand2_sext),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1_sext),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    let nzcv = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    // CMPP discards the result (no register write), only updates flags
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
            mnemonic: "CMPP".to_string(),
            encoding: Some("Compare with tag (alias of SUBPS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmpp_subps.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmpp_validates() {
    let instruction = cmpp_instruction();

    validate_instruction(&instruction).expect("CMPP should validate");
}

#[test]
fn cmpp_writes_flags_only() {
    let instruction = cmpp_instruction();

    // Should only write flags, no register write
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
fn cmpp_uses_sign_extract_and_not_and_set_carry() {
    let instruction = cmpp_instruction();

    // The AddWithCarry should be inside the first WriteFlag's BitTest
    let Stmt::WriteFlag { value, .. } = &instruction.semantics[0] else {
        panic!("expected flag write");
    };

    let Expr::BitTest { value: nzcv, bit: 3 } = value else {
        panic!("expected BitTest for N flag");
    };

    let Expr::TupleElement { value: add_with_carry, index: 1, .. } = nzcv.as_ref() else {
        panic!("expected TupleElement for NZCV");
    };

    let Expr::AddWithCarry { left, right, carry, width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
    assert!(matches!(carry.as_ref(), Expr::Bool(true)));

    // Left operand should be SignExtend(Extract(...))
    let Expr::SignExtend { value: left_extract, to: left_to, .. } = left.as_ref() else {
        panic!("expected SignExtend on left");
    };
    assert_eq!(*left_to, BitWidth::W64);

    let Expr::Extract { lsb, width, .. } = left_extract.as_ref() else {
        panic!("expected Extract inside SignExtend on left");
    };
    assert_eq!(*lsb, 0);
    assert_eq!(*width, BitWidth::new(56).unwrap());

    // Right operand should be Not(SignExtend(Extract(...)))
    let Expr::Unary { op, value: unary_value, .. } = right.as_ref() else {
        panic!("expected Unary Not on right");
    };
    assert_eq!(*op, UnaryOp::Not);

    let Expr::SignExtend { value: right_extract, to: right_to, .. } = unary_value.as_ref() else {
        panic!("expected SignExtend inside Not on right");
    };
    assert_eq!(*right_to, BitWidth::W64);

    let Expr::Extract { lsb, width, .. } = right_extract.as_ref() else {
        panic!("expected Extract inside SignExtend on right");
    };
    assert_eq!(*lsb, 0);
    assert_eq!(*width, BitWidth::new(56).unwrap());
}

#[test]
fn cmpp_requires_feat_mte() {
    let instruction = cmpp_instruction();

    assert!(instruction.required_features.contains(&"FEAT_MTE".to_string()));
}
