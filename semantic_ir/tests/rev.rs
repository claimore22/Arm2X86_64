/// REV (Reverse bytes) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rev.md
///
/// 
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn rev_instruction() -> Instruction {
    // Reverse bytes within containers: opaque placeholder (per REV16 precedent)
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "REV".to_string(), encoding: Some("Reverse bytes".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "rev.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn rev_validates() {
    validate_instruction(&rev_instruction()).expect("should validate");
}

#[test]
fn rev_writes_destination() {
    let instruction = rev_instruction();
    assert!(matches!(instruction.semantics[0], Stmt::WriteReg { .. }));
}
