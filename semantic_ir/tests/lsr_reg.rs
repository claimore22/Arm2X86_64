/// LSR (register): alias of LSRV semantics are representable in the IR.
/// Stanford source: arm64_md_clean/lsr_lsrv.md
///
/// Alias of LSRV. Logical right shift by variable amount.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn lsr_reg_instruction() -> Instruction {
        // LSL/LSR (register) = LSLV/LSRV: Xd = Xn VAR(unsigned(Xm) mod datasize)
    let shifting = Expr::Shift {
        kind: semantic_ir::ShiftKind::Lsr,
        value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        amount: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: shifting }];

    Instruction {
        id: InstructionId {
            mnemonic: "LSR".to_string(),
            encoding: Some("LSR (register): alias of LSRV".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lsr_lsrv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lsr_reg_validates() {
    validate_instruction(&lsr_reg_instruction()).expect("LSR should validate");
}

#[test]
fn lsr_reg_writes_destination() {
    let instruction = lsr_reg_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn lsr_reg_uses_alias_semantics() {
    let instruction = lsr_reg_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Shift { .. }));
}
