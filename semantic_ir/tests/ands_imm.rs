/// ANDS (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ands_log_imm.md
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ands_imm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let operand2 = Expr::Const {
        value: 0x0000FFFF0000FFFFu128, // Example bitmask pattern
        ty: BitWidth::W64,
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(operand1),
        right: Box::new(operand2),
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
            mnemonic: "ANDS".to_string(),
            encoding: Some("Bitwise AND immediate, setting flags".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ands_log_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ands_imm_validates() {
    let instruction = ands_imm_instruction();

    validate_instruction(&instruction).expect("ANDS immediate should validate");
}

#[test]
fn ands_imm_writes_result_and_flags() {
    let instruction = ands_imm_instruction();

    // 1 register write + 4 flag writes = 5 statements
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
fn ands_imm_n_flag_is_msb() {
    let instruction = ands_imm_instruction();

    let Stmt::WriteFlag { flag, value } = &instruction.semantics[1] else {
        panic!("expected WriteFlag for N");
    };

    assert_eq!(*flag, Flag::N);

    let Expr::BitTest { value: bit_value, bit } = value else {
        panic!("expected BitTest for N flag");
    };

    assert_eq!(*bit, 63); // MSB for 64-bit

    // Verify it's testing the result
    assert!(matches!(
        bit_value.as_ref(),
        Expr::Binary {
            op: semantic_ir::BinOp::And,
..
        }
    ));
}

#[test]
fn ands_imm_z_flag_is_zero_test() {
    let instruction = ands_imm_instruction();

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
fn ands_imm_c_and_v_are_zero() {
    let instruction = ands_imm_instruction();

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
