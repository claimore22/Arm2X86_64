/// AESIMC - AES inverse mix columns
/// Stanford source: arm64_md_clean/aesimc_z_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn aesimc_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::AESInvMixColumns,
        predicate: None,
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(BitWidth::W128),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "AESIMC".to_string(),
            encoding: Some("AES inverse mix columns".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_AES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aesimc_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aesimc_sve_validates() {
    let instruction = aesimc_sve_instruction();
    validate_instruction(&instruction).expect("AESIMC SVE should validate");
}