/// BICS (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bics.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, UnaryOp,
    ValueType, Width, validate_instruction,
};

fn bics_shifted_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(Expr::ReadReg {
            reg: Reg::X(3),
            width: BitWidth::W64,
        }),
        amount: Box::new(Expr::Const {
            value: 4,
            ty: BitWidth::W32,
        }),
        ty: BitWidth::W64,
    };

    // BICS: operand1 AND NOT(operand2)
    let not_operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(operand1),
        right: Box::new(not_operand2),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let mut semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result.clone(),
    }];

    // Stanford:
    // PSTATE.[N,Z,C,V] = result[datasize-1]::IsZeroBit(result)::'00'
    //
    // N = result[63] (MSB)
    // Z = IsZeroBit(result) = (result == 0)
    // C = 0
    // V = 0

    // N flag: BitTest(result, 63)
    semantics.push(Stmt::WriteFlag {
        flag: Flag::N,
        value: Expr::BitTest {
            value: Box::new(result.clone()),
            bit: 63,
        },
    });

    // Z flag: result == 0
    semantics.push(Stmt::WriteFlag {
        flag: Flag::Z,
        value: Expr::Binary {
            op: semantic_ir::BinOp::Eq,
            left: Box::new(result.clone()),
            right: Box::new(Expr::Const {
                value: 0,
                ty: BitWidth::W64,
            }),
            ty: ValueType::Bool,
        },
    });

    // C flag: 0
    semantics.push(Stmt::WriteFlag {
        flag: Flag::C,
        value: Expr::Bool(false),
    });

    // V flag: 0
    semantics.push(Stmt::WriteFlag {
        flag: Flag::V,
        value: Expr::Bool(false),
    });

    Instruction {
        id: InstructionId {
            mnemonic: "BICS".to_string(),
            encoding: Some("Bitwise bit clear shifted register, setting flags".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bics.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bics_shifted_register_validates() {
    let instruction = bics_shifted_instruction();

    validate_instruction(&instruction).expect("BICS shifted register should validate");
}

#[test]
fn bics_shifted_register_writes_result_and_flags() {
    let instruction = bics_shifted_instruction();

    assert_eq!(instruction.semantics.len(), 5);

    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));

    for (statement, expected_flag) in [
        (&instruction.semantics[1], Flag::N),
        (&instruction.semantics[2], Flag::Z),
        (&instruction.semantics[3], Flag::C),
        (&instruction.semantics[4], Flag::V),
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
fn bics_shifted_register_n_flag_is_msb() {
    let instruction = bics_shifted_instruction();

    let Stmt::WriteFlag { flag, value } = &instruction.semantics[1] else {
        panic!("expected WriteFlag for N");
    };

    assert_eq!(*flag, Flag::N);

    let Expr::BitTest { value: bit_value, bit } = value else {
        panic!("expected BitTest for N flag");
    };

    assert_eq!(*bit, 63);

    // Verify it's testing the result (which is AND with NOT)
    assert!(matches!(
        bit_value.as_ref(),
        Expr::Binary {
            op: semantic_ir::BinOp::And,
..
        }
    ));
}

#[test]
fn bics_shifted_register_z_flag_is_zero_test() {
    let instruction = bics_shifted_instruction();

    let Stmt::WriteFlag { flag, value } = &instruction.semantics[2] else {
        panic!("expected WriteFlag for Z");
    };

    assert_eq!(*flag, Flag::Z);

    let Expr::Binary { op, left, right, ty } = value else {
        panic!("expected Binary Eq for Z flag");
    };

    assert_eq!(*op, semantic_ir::BinOp::Eq);
    assert_eq!(*ty, ValueType::Bool);

    // Left is the result
    assert!(matches!(
        left.as_ref(),
        Expr::Binary {
            op: semantic_ir::BinOp::And,
..
        }
    ));

    // Right is zero constant
    assert!(matches!(
        right.as_ref(),
        Expr::Const { value: 0, ty: BitWidth::W64 }
    ));
}

#[test]
fn bics_shifted_register_c_and_v_are_zero() {
    let instruction = bics_shifted_instruction();

    for (statement, expected_flag) in [
        (&instruction.semantics[3], Flag::C),
        (&instruction.semantics[4], Flag::V),
    ] {
        let Stmt::WriteFlag { flag, value } = statement else {
            panic!("expected WriteFlag");
        };

        assert_eq!(*flag, expected_flag);
        assert!(matches!(value, Expr::Bool(false)));
    }
}

#[test]
fn bics_shifted_register_uses_and_not() {
    let instruction = bics_shifted_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Binary { op, left, right, .. } = value else {
        panic!("expected Binary And");
    };

    assert_eq!(*op, semantic_ir::BinOp::And);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));

    // Right operand should be NOT(shifted register)
    let Expr::Unary { op: unary_op, value: unary_value, .. } = right.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*unary_op, UnaryOp::Not);

    // The inner value should be a shift
    assert!(matches!(
        unary_value.as_ref(),
        Expr::Shift {
            kind: ShiftKind::Lsl,
..
        }
    ));
}
