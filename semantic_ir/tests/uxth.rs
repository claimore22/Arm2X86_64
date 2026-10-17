/// UXTH (alias of UBFM) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/uxth_ubfm.md
///
/// Zero-extend halfword.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn uxth_instruction() -> Instruction {
    let result = Expr::ZeroExtend {
        value: Box::new(Expr::Extract {
            value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
            lsb: 0,
            width: BitWidth::W16,
        }),
        to: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "UXTH".to_string(), encoding: Some("__TITLE__".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "uxth_ubfm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn uxth_validates() {
    validate_instruction(&uxth_instruction()).expect("should validate");
}
