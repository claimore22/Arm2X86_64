/// BFMLA (vectors) - BFloat16 fused multiply-add (predicated)
/// Stanford source: arm64_md_clean/bfmla_z_p_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn bfmla_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W16;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand3 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Bfloat16Fma,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        middle: Box::new(operand2),
        right: Box::new(operand3),
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 2,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFMLA".to_string(),
            encoding: Some("BFloat16 fused multiply-add (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_B16B16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfmla_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfmla_sve_predicated_validates() {
    let instruction = bfmla_sve_predicated_instruction();
    validate_instruction(&instruction).expect("BFMLA SVE predicated should validate");
}