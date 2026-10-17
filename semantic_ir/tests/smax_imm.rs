/// SMAX (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/smax_imm.md
///
/// Select( Xn > imm, Xn, imm ) signed. FEAT_CSSC.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn smax_imm_instruction() -> Instruction {
    // SMAX (immediate): Xd = signed_max(Xn, imm)
    let cmp = Expr::Binary {
        op: semantic_ir::BinOp::GtSigned,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        ty: ValueType::Bool,
    };
    let result = Expr::Select {
        condition: Box::new(cmp),
        when_true: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        when_false: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SMAX".to_string(), encoding: Some("Signed maximum (immediate)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_CSSC".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "smax_imm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn smax_imm_validates() {
    validate_instruction(&smax_imm_instruction()).expect("should validate");
}
