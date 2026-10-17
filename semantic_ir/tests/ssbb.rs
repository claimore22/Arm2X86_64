/// SSBB (Speculative store bypass barrier: alias of DSB) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ssbb_dsb.md
///
/// SpeculativeStoreBypassBarrierToVA. FEAT_SPECRES2.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn ssbb_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "ssbb".to_string(),
        value: Expr::Const { value: 1, ty: BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "SSBB".to_string(), encoding: Some("Speculative store bypass barrier (DSB alias)".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPECRES2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "ssbb_dsb.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn ssbb_validates() {
    validate_instruction(&ssbb_instruction()).expect("should validate");
}
