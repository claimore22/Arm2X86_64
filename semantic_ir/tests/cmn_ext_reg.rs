/// CMN (extended register) is an alias of ADDS (extended register).
/// Stanford source: arm64_md_clean/cmn_adds_addsub_ext.md
/// It adds register + extended register, updates flags, but discards the result.
use semantic_ir::{
    BitWidth, Expr, ExtendKind, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType,
    Width, validate_instruction,
};

fn cmn_ext_reg_instruction() -> Instruction {
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

    let operand2 = Expr::ExtendReg {
        register: Box::new(Expr::Const {
            value: 3,
            ty: BitWidth::W32,
        }),
        kind: ExtendKind::Uxtw,
        shift: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W32,
        }),
        to: datasize,
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

    // CMN discards the result (no register write), only updates flags
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
            encoding: Some("Compare negative (extended register, alias of ADDS)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmn_adds_addsub_ext.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmn_ext_reg_validates() {
    let instruction = cmn_ext_reg_instruction();

    validate_instruction(&instruction).expect("CMN extended register should validate");
}

#[test]
fn cmn_ext_reg_writes_flags_only() {
    let instruction = cmn_ext_reg_instruction();

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
fn cmn_ext_reg_uses_extend_reg() {
    let instruction = cmn_ext_reg_instruction();

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

    let Expr::AddWithCarry { right, carry, width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
    assert!(matches!(carry.as_ref(), Expr::Bool(false)));

    let Expr::ExtendReg { kind, .. } = right.as_ref() else {
        panic!("expected ExtendReg");
    };

    assert_eq!(*kind, ExtendKind::Uxtw);
}
