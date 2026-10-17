/// STLUR (Store-release register, unscaled) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stlur_gen.md
///
/// Store-release register (unscaled): MemWrite with release ordering with unscaled offset.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn stlur_instruction() -> Instruction {
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
        value: Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 },
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Release,
    }];

    Instruction {
        id: InstructionId { mnemonic: "STLUR".to_string(), encoding: Some("Store-release register (unscaled)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stlur_gen.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stlur_validates() {
    validate_instruction(&stlur_instruction()).expect("should validate");
}

#[test]
fn stlur_is_release_store() {
    let instruction = stlur_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::MemWrite { ordering: MemOrdering::Release, .. }
    ));
}
