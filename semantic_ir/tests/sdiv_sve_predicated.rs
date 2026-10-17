/// SDIV - Signed divide (predicated)
/// Stanford source: arm64_md_clean/sdiv_z_p_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn sdiv_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sdiv,
        predicate: Box::new(predicate), left: Box::new(operand1), right: Box::new(operand2), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "SDIV".to_string(), encoding: Some("Signed divide (predicated)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "sdiv_z_p_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn sdiv_sve_predicated_validates() { validate_instruction(&sdiv_sve_predicated_instruction()).expect("SDIV SVE predicated should validate"); }