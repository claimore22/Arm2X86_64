/// SMAXV - Signed maximum reduction to scalar
/// Stanford source: arm64_md_clean/smaxv_r_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn smaxv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::SMaxV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: result }];
    Instruction {
        id: InstructionId { mnemonic: "SMAXV".to_string(), encoding: Some("Signed maximum reduction to scalar".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "smaxv_r_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn smaxv_sve_validates() { validate_instruction(&smaxv_sve_instruction()).expect("SMAXV SVE should validate"); }