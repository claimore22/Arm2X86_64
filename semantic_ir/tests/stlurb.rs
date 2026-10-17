/// STLURB (Store-release register byte, unscaled) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stlurb.md
///
/// Store-release register byte (unscaled): MemWrite with release ordering.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stlurb_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    // unscaled signed offset
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 4, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::MemWrite {
        address,
        value: Expr::ReadReg { reg: Reg::W(2), width: BitWidth::W8 },
        size: MemSize::Bits8,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    }];

    Instruction {
        id: InstructionId { mnemonic: "STLURB".to_string(), encoding: Some("Store-release register byte (unscaled)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stlurb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stlurb_validates() {
    validate_instruction(&stlurb_instruction()).expect("should validate");
}

#[test]
fn stlurb_is_release_store() {
    let instruction = stlurb_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
