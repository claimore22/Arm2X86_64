/// FSUBR (vectors) - Floating-point reversed subtract (predicated)
/// Stanford source: arm64_md_clean/fsubr_z_p_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn fsubr_sve_vectors_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fsub,
        predicate: Box::new(predicate), left: Box::new(operand2), right: Box::new(operand1), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "FSUBR".to_string(), encoding: Some("Floating-point reversed subtract (vectors)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "fsubr_z_p_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn fsubr_sve_vectors_validates() { validate_instruction(&fsubr_sve_vectors_instruction()).expect("FSUBR SVE vectors should validate"); }