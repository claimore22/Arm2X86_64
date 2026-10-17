/// MNEG: alias of MSUB semantics are representable in the IR.
/// Stanford source: arm64_md_clean/mneg_msub.md
///
/// Alias of MSUB (negate = XZR - Xn*Xm). Multiply-negate.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn mneg_instruction() -> Instruction {
        // MNEG = MSUB Xd, Xn, Xm, XZR: Xd = 0 - Xn*Xm
    let product = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        right: Box::new(product),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "MNEG".to_string(),
            encoding: Some("MNEG: alias of MSUB".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mneg_msub.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mneg_validates() {
    validate_instruction(&mneg_instruction()).expect("MNEG should validate");
}

#[test]
fn mneg_writes_destination() {
    let instruction = mneg_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn mneg_uses_alias_semantics() {
    let instruction = mneg_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Binary { op: semantic_ir::BinOp::Sub, .. }));
}
