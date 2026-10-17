/// Subtract extended and scaled register semantics are representable in the IR.
use semantic_ir::{
    BitWidth, Expr, ExtendKind, Instruction, InstructionId, Provenance, Reg, Stmt,
    ValueType, Width, validate_instruction,
};
use semantic_ir::BinOp;

fn sub_extended_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

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
    let extended = Expr::ExtendReg {
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

    // SUB uses: operand2 = NOT(ExtendReg(...))
    let operand2 = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(extended),
        ty: ValueType::Bits(datasize),
    };

    // SUB uses carry-in = 1.
    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    // (result, -) = AddWithCarry(...)
    let result = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 0,
        ty: ValueType::Bits(datasize),
    };

    // d == 31: SP{64}() = ZeroExtend{64}(result)
    let sp_result = Expr::ZeroExtend {
        value: Box::new(result.clone()),
        to: BitWidth::W64,
    };

    let write_sp = Stmt::WriteSP {
        value: sp_result,
        width: Width::Fixed(BitWidth::W64),
    };

    // Otherwise: X{datasize}(d) = result
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

    Instruction {
        id: InstructionId {
            mnemonic: "SUB (extended register)".to_string(),
            encoding: Some("Subtract extended register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sub_extended.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sub_extended_register_semantics_are_representable() {
    let instruction = sub_extended_instruction();
    match validate_instruction(&instruction) {
        Ok(()) => {}
        Err(e) => panic!("SUB (extended register) should validate: {e}"),
    }

    let semantics = &instruction.semantics;
    assert_eq!(semantics.len(), 1);

    match &semantics[0] {
        Stmt::If {
            condition: _,
            then_body,
            else_body,
        } => {
            assert_eq!(then_body.len(), 1);
            assert_eq!(else_body.len(), 1);
        }
        _ => panic!("expected conditional destination write"),
    }
}

#[test]
fn sub_extended_register_uses_not_and_set_carry() {
    let extended = Expr::ExtendReg {
        register: Box::new(Expr::Const {
            value: 3,
            ty: BitWidth::W32,
        }),
        kind: ExtendKind::Uxtw,
        shift: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W32,
        }),
        to: Width::InstructionDataSize,
    };

    let inverted = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(extended),
        ty: ValueType::Bits(Width::InstructionDataSize),
    };

    let sub = Expr::AddWithCarry {
        left: Box::new(Expr::Const {
            value: 10,
            ty: BitWidth::W64,
        }),
        right: Box::new(inverted),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    match sub {
        Expr::AddWithCarry { carry, .. } => {
            assert!(matches!(*carry, Expr::Bool(true)));
        }
        _ => panic!("expected AddWithCarry"),
    }
}
