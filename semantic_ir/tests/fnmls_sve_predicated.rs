/// FNMLS - Floating-point negated fused multiply-subtract (predicated)
/// Stanford source: arm64_md_clean/fnmls_z_p_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn fnmls_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };
    let operand3 = Expr::ReadScalableVecReg { reg: 3, vl };
    let destination = Expr::ReadScalableVecReg { reg: 3, vl };

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fnmsub,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        middle: Box::new(operand2),
        right: Box::new(operand3),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 3, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FNMLS".to_string(),
            encoding: Some("Floating-point negated fused multiply-subtract (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fnmls_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fnmls_sve_predicated_validates() {
    let instruction = fnmls_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FNMLS SVE predicated should validate");
}
