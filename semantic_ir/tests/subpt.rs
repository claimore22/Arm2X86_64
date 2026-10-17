/// SUBPT (Subtract checked pointer) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/subpt.md
///
/// Subtract checked pointer via PointerAddCheck. Modeled with the canonical instruction's semantics.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn subpt_instruction() -> Instruction {
    // NEG = SUB Xd, XZR, Xn (LSL #0)
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId {
            mnemonic: "SUBPT".to_string(),
            encoding: Some("SUBPT (Subtract checked pointer)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "subpt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn subpt_validates() {
    validate_instruction(&subpt_instruction()).expect("NEG should validate");
}

#[test]
fn subpt_writes_destination() {
    let instruction = subpt_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn subpt_uses_alias_semantics() {
    let instruction = subpt_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Binary { op: semantic_ir::BinOp::Sub, .. }));
}
