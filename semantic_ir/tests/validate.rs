use semantic_ir::{
    BinOp, BitWidth, Condition, DecodeExpr, DecodeSpec, DecodeStmt, Exception, Expr, ExtendKind,
    Flag, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Operand, Provenance, Reg,
    SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn provenance() -> Provenance {
    Provenance {
        source_path: "test/add.md".to_owned(),
        source_sha256: "source".to_owned(),
        clean_sha256: "clean".to_owned(),
    }
}

fn instruction_with(semantics: Vec<Stmt>) -> Instruction {
    Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: provenance(),
    }
}

fn valid_const() -> Expr {
    Expr::Const {
        value: 1,
        ty: BitWidth::W32,
    }
}

fn valid_bool() -> Expr {
    Expr::Bool(true)
}

fn valid_add_with_carry(width: BitWidth) -> Expr {
    Expr::AddWithCarry {
        left: Box::new(Expr::Const {
            value: 1,
            ty: width,
        }),
        right: Box::new(Expr::Const {
            value: 2,
            ty: width,
        }),
        carry: Box::new(valid_bool()),
        width,
    }
}

#[test]
fn empty_mnemonic_is_rejected() {
    let mut instruction = instruction_with(vec![Stmt::Discard {
        value: valid_const(),
    }]);

    instruction.id.mnemonic.clear();

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn empty_semantics_are_rejected() {
    let instruction = instruction_with(Vec::new());

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn valid_constant_is_accepted() {
    let instruction = instruction_with(vec![Stmt::Discard {
        value: valid_const(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn constant_outside_width_is_rejected() {
    let instruction = instruction_with(vec![Stmt::Discard {
        value: Expr::Const {
            value: 1u128 << 32,
            ty: BitWidth::W32,
        },
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn boolean_expression_is_accepted() {
    let instruction = instruction_with(vec![Stmt::If {
        condition: valid_bool(),
        then_body: vec![Stmt::Discard {
            value: valid_const(),
        }],
        else_body: Vec::new(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn non_boolean_if_condition_is_rejected() {
    let instruction = instruction_with(vec![Stmt::If {
        condition: valid_const(),
        then_body: Vec::new(),
        else_body: Vec::new(),
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn binary_bitvector_types_must_match() {
    let expression = Expr::Binary {
        op: BinOp::Add,
        left: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn binary_result_type_must_match_operands() {
    let expression = Expr::Binary {
        op: BinOp::Add,
        left: Box::new(valid_const()),
        right: Box::new(valid_const()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn comparison_returns_boolean() {
    let expression = Expr::Binary {
        op: BinOp::Eq,
        left: Box::new(valid_const()),
        right: Box::new(valid_const()),
        ty: ValueType::Bool,
    };

    let instruction = instruction_with(vec![Stmt::If {
        condition: expression,
        then_body: Vec::new(),
        else_body: Vec::new(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn comparison_with_bitvector_result_is_rejected() {
    let expression = Expr::Binary {
        op: BinOp::Eq,
        left: Box::new(valid_const()),
        right: Box::new(valid_const()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn unary_bitvector_operation_is_accepted() {
    let expression = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(valid_const()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn unary_result_type_must_match_operand() {
    let expression = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(valid_const()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn shift_amount_must_be_bitvector() {
    let expression = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(valid_const()),
        amount: Box::new(valid_bool()),
        ty: BitWidth::W32,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn shift_value_and_result_width_must_match() {
    let expression = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(valid_const()),
        amount: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        }),
        ty: BitWidth::W64,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn zero_extend_must_widen() {
    let expression = Expr::ZeroExtend {
        value: Box::new(valid_const()),
        to: BitWidth::W64,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn zero_extend_to_smaller_width_is_rejected() {
    let expression = Expr::ZeroExtend {
        value: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        to: BitWidth::W32,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn sign_extend_must_widen() {
    let expression = Expr::SignExtend {
        value: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        }),
        to: BitWidth::W64,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn fixed_width_extend_is_accepted() {
    let expression = Expr::Extend {
        kind: ExtendKind::Uxtw,
        value: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        }),
        to: BitWidth::W64,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn extend_to_smaller_width_is_rejected() {
    let expression = Expr::Extend {
        kind: ExtendKind::Uxtw,
        value: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        to: BitWidth::W32,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn extend_reg_accepts_instruction_datasize() {
    let expression = Expr::ExtendReg {
        register: Box::new(Expr::Const {
            value: 3,
            ty: BitWidth::W32,
        }),
        kind: ExtendKind::Uxtw,
        shift: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        to: Width::InstructionDataSize,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn extend_reg_register_index_must_be_32_bit() {
    let expression = Expr::ExtendReg {
        register: Box::new(Expr::Const {
            value: 3,
            ty: BitWidth::W64,
        }),
        kind: ExtendKind::Uxtw,
        shift: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        to: Width::Fixed(BitWidth::W64),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn extend_reg_shift_must_be_bitvector() {
    let expression = Expr::ExtendReg {
        register: Box::new(Expr::Const {
            value: 3,
            ty: BitWidth::W32,
        }),
        kind: ExtendKind::Uxtw,
        shift: Box::new(valid_bool()),
        to: Width::Fixed(BitWidth::W64),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn add_with_carry_accepts_boolean_carry() {
    let expression = valid_add_with_carry(BitWidth::W32);

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn add_with_carry_rejects_wrong_operand_width() {
    let expression = Expr::AddWithCarry {
        left: Box::new(valid_const()),
        right: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        carry: Box::new(valid_bool()),
        width: BitWidth::W32,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn add_with_carry_rejects_non_boolean_carry() {
    let expression = Expr::AddWithCarry {
        left: Box::new(valid_const()),
        right: Box::new(valid_const()),
        carry: Box::new(valid_const()),
        width: BitWidth::W32,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn tuple_element_zero_is_add_result() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_add_with_carry(BitWidth::W32)),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn tuple_element_one_is_nzcv() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_add_with_carry(BitWidth::W32)),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn tuple_element_one_is_not_boolean() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_add_with_carry(BitWidth::W32)),
        index: 1,
        ty: ValueType::Bool,
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn tuple_element_zero_must_match_add_width() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_add_with_carry(BitWidth::W64)),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn tuple_element_one_must_be_four_bits() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_add_with_carry(BitWidth::W32)),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn tuple_element_invalid_index_is_rejected() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_add_with_carry(BitWidth::W32)),
        index: 2,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn tuple_element_non_tuple_expression_is_rejected() {
    let expression = Expr::TupleElement {
        value: Box::new(valid_const()),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn select_accepts_matching_branch_types() {
    let expression = Expr::Select {
        condition: Box::new(valid_bool()),
        when_true: Box::new(valid_const()),
        when_false: Box::new(valid_const()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn select_rejects_non_boolean_condition() {
    let expression = Expr::Select {
        condition: Box::new(valid_const()),
        when_true: Box::new(valid_const()),
        when_false: Box::new(valid_const()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn select_rejects_mismatched_branch_types() {
    let expression = Expr::Select {
        condition: Box::new(valid_bool()),
        when_true: Box::new(valid_const()),
        when_false: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };

    let instruction = instruction_with(vec![Stmt::Discard { value: expression }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn read_register_is_valid() {
    let instruction = instruction_with(vec![Stmt::Discard {
        value: Expr::ReadReg {
            reg: Reg::x(0).expect("x0 is valid"),
            width: BitWidth::W64,
        },
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn invalid_register_index_cannot_be_constructed() {
    assert_eq!(Reg::x(31), Some(Reg::Xzr));
    assert_eq!(Reg::w(31), Some(Reg::Wzr));
    assert!(Reg::v(32).is_none());
}

#[test]
fn read_sp_supports_32_and_64_bit_widths() {
    for width in [BitWidth::W32, BitWidth::W64] {
        let instruction = instruction_with(vec![Stmt::Discard {
            value: Expr::ReadSP { width },
        }]);

        assert!(validate_instruction(&instruction).is_ok());
    }
}

#[test]
fn read_sp_rejects_unsupported_width() {
    let instruction = instruction_with(vec![Stmt::Discard {
        value: Expr::ReadSP {
            width: BitWidth::W128,
        },
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn read_flag_is_boolean() {
    let instruction = instruction_with(vec![Stmt::If {
        condition: Expr::ReadFlag { flag: Flag::Z },
        then_body: Vec::new(),
        else_body: Vec::new(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn condition_holds_is_boolean() {
    let instruction = instruction_with(vec![Stmt::If {
        condition: Expr::ConditionHolds {
            condition: Condition::Eq,
        },
        then_body: Vec::new(),
        else_body: Vec::new(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn write_flag_requires_boolean_value() {
    let instruction = instruction_with(vec![Stmt::WriteFlag {
        flag: Flag::N,
        value: valid_const(),
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn write_flag_accepts_boolean_value() {
    let instruction = instruction_with(vec![Stmt::WriteFlag {
        flag: Flag::N,
        value: valid_bool(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn write_sp_accepts_64_bit_value() {
    let instruction = instruction_with(vec![Stmt::WriteSP {
        value: Expr::ZeroExtend {
            value: Box::new(valid_const()),
            to: BitWidth::W64,
        },
        width: Width::Fixed(BitWidth::W64),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn write_sp_rejects_wrong_value_width() {
    let instruction = instruction_with(vec![Stmt::WriteSP {
        value: valid_const(),
        width: Width::Fixed(BitWidth::W64),
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn write_register_requires_matching_width() {
    let instruction = instruction_with(vec![Stmt::WriteReg {
        reg: Reg::x(0).expect("x0 is valid"),
        value: valid_const(),
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn write_register_accepts_matching_width() {
    let instruction = instruction_with(vec![Stmt::WriteReg {
        reg: Reg::w(0).expect("w0 is valid"),
        value: valid_const(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn write_special_accepts_matching_width() {
    let instruction = instruction_with(vec![Stmt::WriteSpecial {
        reg: SpecialReg::Pc,
        value: Expr::Const {
            value: 0,
            ty: BitWidth::W64,
        },
        width: Width::Fixed(BitWidth::W64),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn discard_accepts_any_valid_expression() {
    let instruction = instruction_with(vec![Stmt::Discard {
        value: Expr::Bool(false),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn mem_read_has_address_and_size() {
    let instruction = instruction_with(vec![Stmt::Discard {
        value: Expr::MemRead {
            address: Expr::Const {
                value: 0x1000,
                ty: BitWidth::W64,
            }
            .into(),
            size: MemSize::Bits32,
        },
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn mem_write_has_explicit_access_and_ordering() {
    let instruction = instruction_with(vec![Stmt::MemWrite {
        address: valid_const(),
        value: valid_const(),
        size: MemSize::Bits32,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn memory_address_must_be_bitvector() {
    let instruction = instruction_with(vec![Stmt::MemWrite {
        address: valid_bool(),
        value: valid_const(),
        size: MemSize::Bits32,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn let_statement_accepts_valid_expression() {
    let instruction = instruction_with(vec![Stmt::Let {
        name: "result".to_owned(),
        value: valid_const(),
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn empty_let_name_is_rejected() {
    let instruction = instruction_with(vec![Stmt::Let {
        name: String::new(),
        value: valid_const(),
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn branch_target_must_be_bitvector() {
    let instruction = instruction_with(vec![Stmt::Branch {
        target: valid_bool(),
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn branch_target_accepts_bitvector() {
    let instruction = instruction_with(vec![Stmt::Branch {
        target: Expr::Const {
            value: 0x1000,
            ty: BitWidth::W64,
        },
    }]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn conditional_branch_requires_boolean_condition() {
    let instruction = instruction_with(vec![Stmt::BranchConditional {
        condition: valid_const(),
        target: Expr::Const {
            value: 0x1000,
            ty: BitWidth::W64,
        },
    }]);

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn raise_exception_is_valid() {
    let instruction = instruction_with(vec![Stmt::RaiseException(Exception::UndefinedInstruction)]);

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn decode_identifier_is_valid() {
    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Let {
                name: "x".to_owned(),
                value: DecodeExpr::Identifier("Rn".to_owned()),
            }],
        }),
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn decode_undefined_statement_is_valid() {
    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Undefined],
        }),
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn decode_in_expression_is_valid() {
    let expression = DecodeExpr::In {
        value: Box::new(DecodeExpr::UInt("imm3".to_owned())),
        values: vec![5, 6, 7],
    };

    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Require {
                condition: expression,
            }],
        }),
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn decode_empty_in_list_is_rejected() {
    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Require {
                condition: DecodeExpr::In {
                    value: Box::new(DecodeExpr::Identifier("x".to_owned())),
                    values: Vec::new(),
                },
            }],
        }),
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn decode_shift_left_is_valid() {
    let expression = DecodeExpr::ShiftLeft {
        value: Box::new(DecodeExpr::UInt("sf".to_owned())),
        amount: 5,
    };

    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Let {
                name: "datasize".to_owned(),
                value: expression,
            }],
        }),
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn provenance_is_required() {
    let mut instruction = instruction_with(vec![Stmt::Discard {
        value: valid_const(),
    }]);

    instruction.provenance.source_path.clear();

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn provenance_hashes_are_required() {
    let mut instruction = instruction_with(vec![Stmt::Discard {
        value: valid_const(),
    }]);

    instruction.provenance.source_sha256.clear();

    assert!(validate_instruction(&instruction).is_err());

    instruction.provenance.source_sha256 = "source".to_owned();
    instruction.provenance.clean_sha256.clear();

    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn operands_can_be_typed() {
    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: vec![
            Operand::Register {
                name: "Rn".to_owned(),
                ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
            },
            Operand::Immediate {
                name: "imm".to_owned(),
                ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
            },
            Operand::Condition {
                name: "cond".to_owned(),
            },
        ],
        required_features: Vec::new(),
        decode: None,
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_ok());
}

#[test]
fn memory_operand_is_representable() {
    let instruction = Instruction {
        id: InstructionId {
            mnemonic: "TEST".to_owned(),
            encoding: None,
        },
        operands: vec![Operand::Memory {
            name: "address".to_owned(),
        }],
        required_features: Vec::new(),
        decode: None,
        semantics: vec![Stmt::Discard {
            value: valid_const(),
        }],
        provenance: provenance(),
    };

    assert!(validate_instruction(&instruction).is_ok());
}
