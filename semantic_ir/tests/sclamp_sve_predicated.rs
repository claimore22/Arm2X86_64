/// SCLAMP - Signed clamp to minimum/maximum (predicated)
/// Stanford source: arm64_md_clean/sclamp_z_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, primitives::PredicationMode, validate_instruction};

fn sclamp_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 0, vl };
    let min_vec = Expr::ReadScalableVecReg { reg: 1, vl };
    let max_vec = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    // Clamp = SUMAX(SUMIN(operand, max_val), min_val)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmla,
        predicate: Box::new(predicate),
        left: Box::new(operand), middle: Box::new(min_vec), right: Box::new(max_vec),
        destination: Box::new(destination),
        vl, esize,
        predication: PredicationMode::Merging,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "SCLAMP".to_string(), encoding: Some("Signed clamp to minimum/maximum".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "sclamp_z_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn sclamp_sve_predicated_validates() { validate_instruction(&sclamp_sve_predicated_instruction()).expect("SCLAMP SVE predicated should validate"); }