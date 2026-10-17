/// BFCLAMP - BFloat16 clamp to minimum/maximum number (unpredicated)
/// Stanford source: arm64_md_clean/bfclamp_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn bfclamp_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W16; // BFloat16 is 16-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // BFCLAMP is a ternary operation: result = BFMinNum(BFMaxNum(op1, op3), op2)
    // For IR modeling, we use a special ternary operation
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::BFClamp,
        predicate: None,
        operands: vec![
            Box::new(Expr::ReadScalableVecReg { reg: 1, vl }),  // min
            Box::new(Expr::ReadScalableVecReg { reg: 2, vl }),  // max
            Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),  // value to clamp
        ],
        vl,
        esize: Some(BitWidth::W16),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFCLAMP".to_string(),
            encoding: Some("BFloat16 clamp to minimum/maximum number (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_B16B16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfclamp_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfclamp_sve_unpredicated_validates() {
    let instruction = bfclamp_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("BFCLAMP SVE should validate");
}