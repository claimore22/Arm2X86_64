/// SHSUB - Signed halving subtract
/// Stanford source: arm64_md_clean/shsub_z_p_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn shsub_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sub,  // SHSUB = (a - b) / 2
        predicate: Box::new(predicate), left: Box::new(operand1), right: Box::new(operand2), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "SHSUB".to_string(), encoding: Some("Signed halving subtract".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "shsub_z_p_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn shsub_sve_predicated_validates() { validate_instruction(&shsub_sve_predicated_instruction()).expect("SHSUB SVE predicated should validate"); }