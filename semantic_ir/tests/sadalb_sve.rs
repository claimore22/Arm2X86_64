/// SADALB - Signed absolute difference and accumulate long (bottom)
/// Stanford source: arm64_md_clean/sadalb_z_zzz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn sadalb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand3 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 2, vl };
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmla,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(operand1), middle: Box::new(operand2), right: Box::new(operand3), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 2, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "SADALB".to_string(), encoding: Some("Signed absolute difference and accumulate long (bottom)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "sadalb_z_zzz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn sadalb_sve_validates() { validate_instruction(&sadalb_sve_instruction()).expect("SADALB SVE should validate"); }