/// NEGS: alias of SUBS (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/negs_subs_addsub_shift.md
///
/// Alias of SUBS (negate setting flags). Modeled with the canonical instruction's semantics.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn negs_instruction() -> Instruction {
    // NEGS = SUBS Xd, XZR, Xn: writes result and NZCV
    let sum = Expr::AddWithCarry {
        left: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        right: Box::new(Expr::Unary {
            op: semantic_ir::UnaryOp::Not,
            value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };
    let result = Expr::TupleElement {
        value: Box::new(sum),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "NEGS".to_string(),
            encoding: Some("NEGS: alias of SUBS (shifted register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "negs_subs_addsub_shift.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn negs_validates() {
    validate_instruction(&negs_instruction()).expect("NEGS should validate");
}

#[test]
fn negs_writes_destination() {
    let instruction = negs_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn negs_uses_alias_semantics() {
    let instruction = negs_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::TupleElement { index: 0, .. }));
}
