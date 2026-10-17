/// AESD (indexed) - Multi-vector AES single round decryption
/// Stanford source: arm64_md_clean/aesd_mz_zzi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn aesd_sve_indexed_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::AESInvSubBytes,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(BitWidth::W128),
        imm: Some(0),
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "AESD".to_string(),
            encoding: Some("AES single round decryption (indexed)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_AES2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aesd_mz_zzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aesd_sve_indexed_validates() {
    let instruction = aesd_sve_indexed_instruction();
    validate_instruction(&instruction).expect("AESD SVE indexed should validate");
}