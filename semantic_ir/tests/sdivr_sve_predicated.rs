/// SDIVR - Signed reversed divide (predicated)
/// Stanford source: arm64_md_clean/sdivr_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sdivr_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn (divisor, also destination)
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zm (dividend)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::SDivR,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SDIVR".to_string(),
            encoding: Some("Signed reversed divide (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sdivr_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sdivr_sve_predicated_validates() {
    let instruction = sdivr_sve_predicated_instruction();
    validate_instruction(&instruction).expect("SDIVR SVE predicated should validate");
}
