/// NGCS: alias of SBCS (negate with carry, setting flags). Writes NZCV.
/// Stanford source: arm64_md_clean/ngcs_sbcs.md
/// Modeled with the canonical instruction's semantics.
/// Alias/model decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn ngcs_instruction() -> Instruction {
    let sum = Expr::AddWithCarry {
        left: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        right: Box::new(Expr::Unary {
            op: semantic_ir::UnaryOp::Not,
            value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
        carry: Box::new(Expr::ReadFlag { flag: semantic_ir::Flag::C }),
        width: BitWidth::W64,
    };
    let result = Expr::TupleElement {
        value: Box::new(sum.clone()),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let nzcv = Expr::TupleElement {
        value: Box::new(sum),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };
    let mut semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];
    for (flag, bit) in [
        (semantic_ir::Flag::N, 3u16),
        (semantic_ir::Flag::Z, 2),
        (semantic_ir::Flag::C, 1),
        (semantic_ir::Flag::V, 0),
    ] {
        semantics.push(Stmt::WriteFlag {
            flag,
            value: Expr::BitTest { value: Box::new(nzcv.clone()), bit },
        });
    }

    Instruction {
        id: InstructionId { mnemonic: "NGCS".to_string(), encoding: Some("Negate with carry, setting flags (alias of SBCS)".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "ngcs_sbcs.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn ngcs_validates() {
    validate_instruction(&ngcs_instruction()).expect("should validate");
}
