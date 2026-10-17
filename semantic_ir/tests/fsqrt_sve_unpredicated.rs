/// FSQRT (unpredicated) - Floating-point square root (unpredicated)
/// Stanford source: arm64_md_clean/fsqrt_z_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn fsqrt_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fsqrt,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        operand: Box::new(operand), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "FSQRT".to_string(), encoding: Some("Floating-point square root (unpredicated)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "fsqrt_z_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn fsqrt_sve_unpredicated_validates() { validate_instruction(&fsqrt_sve_unpredicated_instruction()).expect("FSQRT SVE unpredicated should validate"); }