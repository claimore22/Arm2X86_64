/// SUB (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sub_addsub_imm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, UnaryOp, ValueType, Width,
    validate_instruction,
};

fn sub_imm_instruction() -> Instruction {
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

    // operand2 = ZeroExtend(imm)
    let imm_bits = BitWidth::new(24).unwrap();
    let imm = Expr::ZeroExtend {
        value: Box::new(Expr::Const {
            value: 0x123456,
            ty: imm_bits,
        }),
        to: BitWidth::W64,
    };

    // SUB: operand2 = NOT(ZeroExtend(imm))
    let operand2 = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1),
        right: Box::new(operand2),
        carry: Box::new(Expr::Bool(true)), // carry-in = 1
        width: BitWidth::W64,
    };

let result = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // if d == 31 then SP{64}() = ZeroExtend{64}(result) else X{datasize}(d) = result
    let sp_result = Expr::ZeroExtend {
        value: Box::new(result.clone()),
        to: BitWidth::W64,
    };

    let write_sp = Stmt::WriteSP {
        value: sp_result,
        width: Width::Fixed(BitWidth::W64),
    };

    let write_reg = Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    };

    let dest_is_sp = Expr::Binary {
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

    let semantics = vec![Stmt::If {
        condition: dest_is_sp,
        then_body: vec![write_sp],
        else_body: vec![write_reg],
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SUB".to_string(),
            encoding: Some("Subtract immediate".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sub_addsub_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sub_imm_validates() {
    let instruction = sub_imm_instruction();

    validate_instruction(&instruction).expect("SUB immediate should validate");
}

#[test]
fn sub_imm_conditional_destination() {
    let instruction = sub_imm_instruction();

    let Stmt::If { condition, then_body, else_body } = &instruction.semantics[0] else {
        panic!("expected If for conditional destination");
    };

    assert!(matches!(condition, Expr::Binary { op: semantic_ir::BinOp::Eq, .. }));
    assert_eq!(then_body.len(), 1);
    assert!(matches!(then_body[0], Stmt::WriteSP { .. }));
    assert_eq!(else_body.len(), 1);
    assert!(matches!(else_body[0], Stmt::WriteReg { reg: Reg::X(1), .. }));
}

#[test]
fn sub_imm_uses_not_and_set_carry() {
    let instruction = sub_imm_instruction();

    let Stmt::If { else_body, .. } = &instruction.semantics[0] else {
        panic!("expected If");
    };

    let Stmt::WriteReg { value, .. } = &else_body[0] else {
        panic!("expected WriteReg");
    };

    let Expr::TupleElement {
        value: add_with_carry,
        index: 0,
..
    } = value
    else {
        panic!("expected TupleElement");
    };

    let Expr::AddWithCarry { right, carry, width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
    assert!(matches!(carry.as_ref(), Expr::Bool(true)));

    // Right operand = NOT(ZeroExtend(imm))
    let Expr::Unary { op, value: unary_value, .. } = right.as_ref() else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, UnaryOp::Not);

    let Expr::ZeroExtend { to, .. } = unary_value.as_ref() else {
        panic!("expected ZeroExtend inside Not");
    };

    assert_eq!(*to, BitWidth::W64);
}

#[test]
fn sub_imm_sp_write_zero_extends() {
    let instruction = sub_imm_instruction();

    let Stmt::If { then_body, .. } = &instruction.semantics[0] else {
        panic!("expected If");
    };

    let Stmt::WriteSP { value, width } = &then_body[0] else {
        panic!("expected WriteSP");
    };

    assert_eq!(*width, Width::Fixed(BitWidth::W64));

    // Result is ZeroExtended before writing to SP
    let Expr::ZeroExtend { value: zero_extend_value, to } = value else {
        panic!("expected ZeroExtend for SP write");
    };

    assert_eq!(*to, BitWidth::W64);

    let Expr::TupleElement { value: add_with_carry, index: 0, .. } = zero_extend_value.as_ref() else {
        panic!("expected TupleElement inside ZeroExtend");
    };

    let Expr::AddWithCarry { width, .. } = add_with_carry.as_ref() else {
        panic!("expected AddWithCarry");
    };

    assert_eq!(*width, BitWidth::W64);
}
