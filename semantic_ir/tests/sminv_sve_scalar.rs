/// SMINV - Signed minimum reduction to scalar
/// Stanford source: arm64_md_clean/sminv_r_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn sminv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::SMinV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };
    let semantics = vec![Stmt::WriteReg { reg: semantic_ir::Reg::X(0), value: result }];
    Instruction {
        id: InstructionId { mnemonic: "SMINV".to_string(), encoding: Some("Signed minimum reduction to scalar".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "sminv_r_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn sminv_sve_validates() { validate_instruction(&sminv_sve_instruction()).expect("SMINV SVE should validate"); }