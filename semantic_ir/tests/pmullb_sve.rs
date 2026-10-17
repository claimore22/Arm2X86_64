/// PMULLB - Polynomial multiply long (bottom)
/// Stanford source: arm64_md_clean/pmullb_z_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn pmullb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Mul,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(operand1), right: Box::new(operand2), destination: Box::new(Expr::Const { value: 0, ty: BitWidth::W256 }),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "PMULLB".to_string(), encoding: Some("Polynomial multiply long (bottom)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "pmullb_z_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn pmullb_sve_validates() { validate_instruction(&pmullb_sve_instruction()).expect("PMULLB SVE should validate"); }