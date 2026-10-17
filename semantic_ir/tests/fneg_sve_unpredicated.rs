/// FNEG (unpredicated) - Floating-point negate (unpredicated)
/// Stanford source: arm64_md_clean/fneg_z_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn fneg_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fneg,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        operand: Box::new(operand), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "FNEG".to_string(), encoding: Some("Floating-point negate (unpredicated)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "fneg_z_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn fneg_sve_unpredicated_validates() { validate_instruction(&fneg_sve_unpredicated_instruction()).expect("FNEG SVE unpredicated should validate"); }