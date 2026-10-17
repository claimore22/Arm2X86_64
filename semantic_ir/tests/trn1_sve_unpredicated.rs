/// TRN1 (vectors) - Interleave even elements from two vectors (unpredicated)
/// Stanford source: arm64_md_clean/trn1_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn trn1_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // 128-bit elements

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Trn1,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(esize),
        imm: Some(1), // part = 1 for TRN1 (even elements)
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "TRN1".to_string(),
            encoding: Some("Interleave even elements from two vectors (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_F64MM".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "trn1_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn trn1_sve_unpredicated_validates() {
    let instruction = trn1_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("TRN1 SVE unpredicated should validate");
}