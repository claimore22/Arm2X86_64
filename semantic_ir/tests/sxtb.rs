/// SXTB (alias of SBFM) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sxtb_sbfm.md
///
/// Sign-extend byte: SignExtend(Extract(0,8)).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn sxtb_instruction() -> Instruction {
    let result = Expr::SignExtend {
        value: Box::new(Expr::Extract {
            value: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
            lsb: 0,
            width: BitWidth::W8,
        }),
        to: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(1), value: result }];

    Instruction {
        id: InstructionId { mnemonic: "SXTB".to_string(), encoding: Some("__TITLE__".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sxtb_sbfm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sxtb_validates() {
    validate_instruction(&sxtb_instruction()).expect("should validate");
}
