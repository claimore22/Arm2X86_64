/// MOV (inverted wide immediate): alias of MOVN semantics are representable in the IR.
/// Stanford source: arm64_md_clean/mov_movn.md
///
/// Alias of MOVN. Inverted wide immediate.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn mov_movn_instruction() -> Instruction {
        // MOV (inverted wide immediate) = MOVN: Xd = NOT(imm16 << (hw*16))
    let imm = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(Expr::Const { value: 0x1234, ty: BitWidth::W64 }),
        amount: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let result = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOV".to_string(),
            encoding: Some("MOV (inverted wide immediate): alias of MOVN".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mov_movn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mov_movn_validates() {
    validate_instruction(&mov_movn_instruction()).expect("MOV should validate");
}

#[test]
fn mov_movn_writes_destination() {
    let instruction = mov_movn_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn mov_movn_uses_alias_semantics() {
    let instruction = mov_movn_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Unary { op: semantic_ir::UnaryOp::Not, .. }));
}
