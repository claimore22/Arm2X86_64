//smoke test for semantic_ir crate
use semantic_ir::{
    BinOp, BitWidth, Condition, DecodeExpr, DecodeSpec, DecodeStmt, Expr, ExtendKind, Flag,
    Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Operand, Provenance, Reg, Stmt,
    ValueType, Width,
};

#[test]
fn add_extended_register_semantics_are_representable() {
    // Stanford:
    //
    // operand1 = if n == 31 then SP{datasize}() else X{datasize}(n)
    // operand2 = ExtendReg(m, extend_type, shift)
    // (result, -) = AddWithCarry(operand1, operand2, '0)
    //
    // if d == 31 then
    //     SP{64}() = ZeroExtend{64}(result)
    // else
    //     X{datasize}(d) = result
    // end

    let datasize = Width::InstructionDataSize;

    // n == 31 selects SP.
    let n_is_sp = Expr::Binary {
        op: BinOp::Eq,
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
        when_false: Box::new(Expr::ReadIndexedReg {
            index: Box::new(Expr::Const {
                value: 2,
                ty: BitWidth::W32,
            }),
            width: BitWidth::W64,
        }),
        ty: ValueType::Bits(datasize),
    };

    // ExtendReg(m, extend_type, shift)
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

    // (result, carry_out) = AddWithCarry(...)
    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(false)),
        width: BitWidth::W64,
    };

    // Select result and discard carry.
    let result = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 0,
        ty: ValueType::Bits(datasize),
    };

    // d == 31 -> SP = ZeroExtend(result)
    let write_sp = Stmt::WriteSP {
        value: Expr::ZeroExtend {
            value: Box::new(result.clone()),
            to: BitWidth::W64,
        },
        width: Width::Fixed(BitWidth::W64),
    };

    // d != 31 -> X[d] = result
    let write_register = Stmt::WriteIndexedReg {
        index: Expr::Const {
            value: 4,
            ty: BitWidth::W32,
        },
        value: result,
        width: datasize,
    };

    let destination_is_sp = Expr::Binary {
        op: BinOp::Eq,
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

    let semantics = vec![Stmt::If {
        condition: destination_is_sp,
        then_body: vec![write_sp],
        else_body: vec![write_register],
    }];

    assert_eq!(semantics.len(), 1);

    assert!(matches!(
        semantics.as_slice(),
        [Stmt::If {
            then_body,
            else_body,
..
        }] if then_body.len() == 1 && else_body.len() == 1
    ));
}

#[test]
fn add_extended_register_decode_can_represent_undefined_constraint() {
    // if imm3 IN {'101', '110', '111'} then
    //     EndOfDecode(Decode_UNDEF);
    // end;

    let decode = DecodeSpec {
        statements: vec![
            DecodeStmt::Require {
                condition: DecodeExpr::Not {
                    value: Box::new(DecodeExpr::In {
                        value: Box::new(DecodeExpr::Identifier("imm3".to_owned())),
                        values: vec![5, 6, 7],
                    }),
                },
            },
            DecodeStmt::Undefined,
        ],
    };

    assert_eq!(decode.statements.len(), 2);
}

#[test]
fn add_extended_register_uses_explicit_extend_kind() {
    let operand = Expr::ExtendReg {
        register: Box::new(Expr::Const {
            value: 7,
            ty: BitWidth::W32,
        }),
        kind: ExtendKind::Sxtw,
        shift: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        to: Width::InstructionDataSize,
    };

    assert!(matches!(
        operand,
        Expr::ExtendReg {
            kind: ExtendKind::Sxtw,
..
        }
    ));
}

#[test]
fn add_with_carry_result_is_explicitly_selected() {
    let add = Expr::AddWithCarry {
        left: Box::new(Expr::Const {
            value: 10,
            ty: BitWidth::W64,
        }),
        right: Box::new(Expr::Const {
            value: 20,
            ty: BitWidth::W64,
        }),
        carry: Box::new(Expr::Bool(false)),
        width: BitWidth::W64,
    };

    let result = Expr::TupleElement {
        value: Box::new(add),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    assert!(matches!(result, Expr::TupleElement { index: 0, .. }));
}

#[test]
fn flags_are_explicit() {
    let flag = Expr::ReadFlag { flag: Flag::Z };

    let condition = Expr::ConditionHolds {
        condition: Condition::Eq,
    };

    let statement = Stmt::WriteFlag {
        flag: Flag::Z,
        value: flag,
    };

    assert!(matches!(
        condition,
        Expr::ConditionHolds {
            condition: Condition::Eq
        }
    ));

    assert!(matches!(statement, Stmt::WriteFlag { flag: Flag::Z, .. }));
}

#[test]
fn memory_semantics_are_explicit() {
    let address = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    let write = Stmt::MemWrite {
        address,
        value: Expr::Const {
            value: 0,
            ty: BitWidth::W32,
        },
        size: MemSize::Bits32,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    };

    assert!(matches!(write, Stmt::MemWrite { .. }));
}

#[test]
fn instruction_has_decode_and_provenance() {
    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "ADD".to_owned(),
            encoding: None,
        },
        operands: vec![Operand::Register {
            name: "Rd".to_owned(),
            ty: ValueType::Bits(Width::InstructionDataSize),
        }],
        required_features: Vec::new(),
        decode: None,
        semantics: Vec::new(),
        provenance: Provenance {
            source_path: "test/smoke.md".to_owned(),
            source_sha256: "source".to_owned(),
            clean_sha256: "clean".to_owned(),
        },
    };

    assert_eq!(instruction.id.mnemonic, "ADD");
}

#[test]
fn arithmetic_expression_is_typed() {
    let expression = Expr::Binary {
        op: BinOp::Add,
        left: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        right: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    assert!(matches!(expression, Expr::Binary { .. }));
}
