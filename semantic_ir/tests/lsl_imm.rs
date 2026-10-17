/// LSL (immediate): alias of UBFM semantics are representable in the IR.
/// Stanford source: arm64_md_clean/lsl_ubfm.md
///
/// Alias of UBFM (alias: LSL = UBFM with immr = (-shift) mod datasize, imms = 63-shift). Logical left shift by immediate.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn lsl_imm_instruction() -> Instruction {
        // LSL/LSR (immediate) = UBFM-style extract: copy of Xm bits, shifted
    let shifting = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsl,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        amount: Box::new(Expr::Const { value: 4, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: shifting }];

    Instruction {
        id: InstructionId {
            mnemonic: "LSL".to_string(),
            encoding: Some("LSL (immediate): alias of UBFM".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lsl_ubfm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lsl_imm_validates() {
    validate_instruction(&lsl_imm_instruction()).expect("LSL should validate");
}

#[test]
fn lsl_imm_writes_destination() {
    let instruction = lsl_imm_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn lsl_imm_uses_alias_semantics() {
    let instruction = lsl_imm_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Shift { .. }));
}
