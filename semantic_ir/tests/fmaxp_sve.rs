/// FMAXP - Floating-point maximum pairwise (predicated)
/// Stanford source: arm64_md_clean/fmaxp_z_p_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, primitives::PredicationMode, validate_instruction};

fn fmaxp_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorPairwise {
        op: semantic_ir::primitives::SVEVectorPairwiseOp::FMaxP,
        predicate: Box::new(predicate), left: Box::new(operand1), right: Box::new(operand2), destination: Box::new(destination),
        vl, esize,
        predication: PredicationMode::Merging,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "FMAXP".to_string(), encoding: Some("Floating-point maximum pairwise".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "fmaxp_z_p_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn fmaxp_sve_validates() { validate_instruction(&fmaxp_sve_instruction()).expect("FMAXP SVE should validate"); }