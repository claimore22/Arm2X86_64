/// SABALB - Signed absolute difference and accumulate long (bottom)
/// Stanford source: arm64_md_clean/sabalb_z_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sabalb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;  // SABALB works on 64-bit elements (bottom half)

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand3 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmla,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        middle: Box::new(operand2),
        right: Box::new(operand3),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 2, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABALB".to_string(),
            encoding: Some("Signed absolute difference and accumulate long (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabalb_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sabalb_sve_validates() {
    let instruction = sabalb_sve_instruction();
    validate_instruction(&instruction).expect("SABALB SVE should validate");
}