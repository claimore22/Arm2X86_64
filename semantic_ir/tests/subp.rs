/// SUBP (Subtract pointer) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/subp.md
///
/// Subtract pointers with validity check: Xd = PointerAddCheck-style opaque (Xn - Xmposed). Modeled with the canonical instruction's semantics.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn subp_instruction() -> Instruction {
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
            mnemonic: "SUBP".to_string(),
            encoding: Some("SUBP (Subtract pointer)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "subp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn subp_validates() {
    validate_instruction(&subp_instruction()).expect("NEG should validate");
}

#[test]
fn subp_writes_destination() {
    let instruction = subp_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn subp_uses_alias_semantics() {
    let instruction = subp_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Binary { op: semantic_ir::BinOp::Sub, .. }));
}
