/// SBFX (alias of SBFM) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sbfx_sbfm.md
///
/// Alias of SBFM. Extracts a signed field. Modeled as a bitfield extract composition
/// (Shift Ror + SignExtend), the canonical SBFM semantics.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn sbfx_instruction() -> Instruction {
    // operand rotated right by immr, then sign-extended from (imms+1) bits
    let rotated = Expr::Shift {
        kind: semantic_ir::ShiftKind::Ror,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        amount: Box::new(Expr::Const { value: 8, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let extracted = Expr::Extract {
        value: Box::new(rotated),
        lsb: 0,
        width: BitWidth::W32,
    };
    let result = Expr::SignExtend {
        value: Box::new(extracted),
        to: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SBFX".to_string(), encoding: Some("Signed bitfield extract".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sbfx_sbfm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sbfx_validates() {
    validate_instruction(&sbfx_instruction()).expect("SBFX should validate");
}

#[test]
fn sbfx_sign_extends_extracted_field() {
    let instruction = sbfx_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::SignExtend { .. }));
}
