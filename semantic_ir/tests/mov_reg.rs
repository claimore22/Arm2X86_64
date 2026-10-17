/// MOV (register): alias of ORR (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/mov_orr_log_shift.md
///
/// Alias of ORR (shifted register). Register move (ORR with XZR).
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn mov_reg_instruction() -> Instruction {
        // MOV (register) = ORR Xd, XZR, Xm (LSL #0)
    let src = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let zero = Expr::Const { value: 0, ty: BitWidth::W64 };
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(zero),
        right: Box::new(src),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOV".to_string(),
            encoding: Some("MOV (register): alias of ORR (shifted register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mov_orr_log_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mov_reg_validates() {
    validate_instruction(&mov_reg_instruction()).expect("MOV should validate");
}

#[test]
fn mov_reg_writes_destination() {
    let instruction = mov_reg_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn mov_reg_uses_alias_semantics() {
    let instruction = mov_reg_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Binary { op, .. } = value else { panic!("expected Binary") };
    assert_eq!(*op, semantic_ir::BinOp::Or);
}
