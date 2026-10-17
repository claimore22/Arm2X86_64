/// MOV (wide immediate): alias of MOVZ semantics are representable in the IR.
/// Stanford source: arm64_md_clean/mov_movz.md
///
/// Alias of MOVZ. Wide immediate.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn mov_movz_instruction() -> Instruction {
        // MOV (wide immediate) = MOVZ: Xd = imm16 << (hw*16), rest zeroed
    let imm = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(Expr::Const { value: 0x1234, ty: BitWidth::W64 }),
        amount: Box::new(Expr::Const { value: 16, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: imm }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOV".to_string(),
            encoding: Some("MOV (wide immediate): alias of MOVZ".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mov_movz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mov_movz_validates() {
    validate_instruction(&mov_movz_instruction()).expect("MOV should validate");
}

#[test]
fn mov_movz_writes_destination() {
    let instruction = mov_movz_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn mov_movz_uses_alias_semantics() {
    let instruction = mov_movz_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Shift { .. }));
}
