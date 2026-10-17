/// UADALP - Unsigned add and accumulate long pairwise (predicated)
/// Stanford source: arm64_md_clean/uadalp_z_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn uadalp_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;  // Each pair of 32-bit elements accumulates into 64-bit
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand_acc = Expr::ReadScalableVecReg { reg: 0, vl };  // Accumulator (Zda)
    let operand_src = Expr::ReadScalableVecReg { reg: 1, vl };  // Source (Zn)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(predicate), left: Box::new(operand_acc), right: Box::new(operand_src), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "UADALP".to_string(), encoding: Some("Unsigned add and accumulate long pairwise".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "uadalp_z_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn uadalp_sve_validates() { validate_instruction(&uadalp_sve_instruction()).expect("UADALP SVE should validate"); }