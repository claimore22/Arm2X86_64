/// MOV (bitmask immediate): alias of ORR (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/mov_orr_log_imm.md
///
/// Alias of ORR (immediate). Bitmask immediate OR with the zero register.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn mov_imm_instruction() -> Instruction {
        // MOV (bitmask immediate) = ORR Wd/Xd, WZR/XZR, #imm (bitmask)
    let operand = Expr::ReadReg { reg: Reg::X(31), width: BitWidth::W64 }; // XZR reads as 0
    let imm = Expr::Const { value: 0xFF00FF00FF00FF00, ty: BitWidth::W64 };
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(operand),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOV".to_string(),
            encoding: Some("MOV (bitmask immediate): alias of ORR (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mov_orr_log_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mov_imm_validates() {
    validate_instruction(&mov_imm_instruction()).expect("MOV should validate");
}

#[test]
fn mov_imm_writes_destination() {
    let instruction = mov_imm_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn mov_imm_uses_alias_semantics() {
    let instruction = mov_imm_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Binary { op, .. } = value else { panic!("expected Binary") };
    assert_eq!(*op, semantic_ir::BinOp::Or);
}
