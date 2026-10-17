/// XPACD (Strip PAC, data) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/xpac.md
///
/// Strips the pointer authentication code from an address, leaving the
/// (sign-extended) address. Modeled as a placeholder composition (masking the
/// top byte range), FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn xpacd_instruction() -> Instruction {
    let src = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    // Clear PAC bits (mask keeps canonical low address bits; sign handled in lowering)
    let cleared = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(src),
        right: Box::new(Expr::Const { value: 0x007F_FFFF_FFFF_FFFF, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: cleared }];

    Instruction {
        id: InstructionId { mnemonic: "XPACD".to_string(), encoding: Some("Strip PAC, data address".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "xpac.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn xpacd_validates() {
    validate_instruction(&xpacd_instruction()).expect("should validate");
}

#[test]
fn xpacd_requires_pauth() {
    assert!(xpacd_instruction()
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
}
