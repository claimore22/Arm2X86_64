/// TST (shifted register): alias of ANDS (shifted register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/tst_ands_log_shift.md
///
/// ANDS to ZR: flags set, result discarded.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn tst_reg_instruction() -> Instruction {
    // TST = ANDS with destination ZR: compute AND, write NZCV, discard result
    let sum = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }),
        right: Box::new(Expr::ReadReg { reg: Reg::X(3), width: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let nzcv = Expr::Const { value: 0, ty: BitWidth::W4 };
    let mut semantics = vec![Stmt::Discard { value: sum }];
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
        id: InstructionId { mnemonic: "TST".to_string(), encoding: Some("__TITLE__".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "tst_ands_log_shift.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn tst_reg_validates() {
    validate_instruction(&tst_reg_instruction()).expect("should validate");
}
