/// ADD (shifted register) - Add optionally-shifted register
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};
use semantic_ir::BinOp;
use semantic_ir::primitives::ShiftKind;

fn add_shifted_instruction() -> Instruction {
    let width = BitWidth::W64;
    let operand1 = Expr::ReadReg { reg: Reg::X(1), width };
    let operand2 = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width }),
        amount: Box::new(Expr::Const { value: 2, ty: BitWidth::W6 }),
        ty: width,
    };
    let result = Expr::Binary {
        op: BinOp::Add,
        left: Box::new(operand1),
        right: Box::new(operand2),
        ty: ValueType::Bits(Width::Fixed(width)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADD (shifted register)".to_string(),
            encoding: Some("Add optionally-shifted register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "add_shifted.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn add_shifted_validates() {
    let instruction = add_shifted_instruction();
    validate_instruction(&instruction).expect("ADD (shifted register) should validate");
}
