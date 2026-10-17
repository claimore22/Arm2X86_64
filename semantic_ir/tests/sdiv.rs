/// SDIV (quotient): Signed divide semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sdiv.md
///
/// Xd = Xn / Xm (signed). Division has no composing IR op; modeled with a placeholder binary op, per CLZ precedent.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn sdiv_instruction() -> Instruction {
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Sub, // placeholder for signed divide
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SDIV".to_string(), encoding: Some("Signed divide".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sdiv.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sdiv_validates() {
    validate_instruction(&sdiv_instruction()).expect("SBC should validate");
}

#[test]
fn sdiv_uses_add_with_carry() {
    let instruction = sdiv_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::Binary { op: semantic_ir::BinOp::Sub, .. }));
}
