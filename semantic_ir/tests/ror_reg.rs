/// ROR (register): alias of RORV semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ror_rorv.md
///
/// 
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ror_reg_instruction() -> Instruction {
    // ROR (register) = RORV Xd, Xn, Xm
    let result = Expr::Shift {
        kind: semantic_ir::ShiftKind::Ror,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        amount: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "ROR".to_string(), encoding: Some("Rotate right (register)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "ror_rorv.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn ror_reg_validates() {
    validate_instruction(&ror_reg_instruction()).expect("should validate");
}

#[test]
fn ror_reg_writes_destination() {
    let instruction = ror_reg_instruction();
    assert!(matches!(instruction.semantics[0], Stmt::WriteReg { .. }));
}
