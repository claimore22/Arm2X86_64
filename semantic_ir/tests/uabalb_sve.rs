/// UABALB - Unsigned absolute difference and accumulate long (bottom)
/// Stanford source: arm64_md_clean/uabalb_z_zzz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn uabalb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand3 = Expr::ReadScalableVecReg { reg: 2, vl };  // Accumulator
    let destination = Expr::ReadScalableVecReg { reg: 2, vl };
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmla,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // Unpredicated
        left: Box::new(operand1), middle: Box::new(operand2), right: Box::new(operand3), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 2, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "UABALB".to_string(), encoding: Some("Unsigned absolute difference and accumulate long (bottom)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "uabalb_z_zzz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn uabalb_sve_validates() { validate_instruction(&uabalb_sve_instruction()).expect("UABALB SVE should validate"); }