/// SBFIZ (alias of SBFM) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sbfiz_sbfm.md
///
/// Alias of SBFM. Inserts into zeros at LSL position. Modeled as a bitfield extract composition
/// (Shift Ror + SignExtend), the canonical SBFM semantics.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn sbfiz_instruction() -> Instruction {
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
        id: InstructionId { mnemonic: "SBFIZ".to_string(), encoding: Some("Signed bitfield insert in zeros".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sbfiz_sbfm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sbfiz_validates() {
    validate_instruction(&sbfiz_instruction()).expect("SBFIZ should validate");
}

#[test]
fn sbfiz_sign_extends_extracted_field() {
    let instruction = sbfiz_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::SignExtend { .. }));
}
