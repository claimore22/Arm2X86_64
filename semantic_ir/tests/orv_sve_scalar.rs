/// ORV - Bitwise inclusive OR reduction to scalar
/// Stanford source: arm64_md_clean/orv_r_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn orv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::OrV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };
    let semantics = vec![Stmt::WriteReg { reg: semantic_ir::Reg::X(0), value: result }];
    Instruction {
        id: InstructionId { mnemonic: "ORV".to_string(), encoding: Some("Bitwise inclusive OR reduction to scalar".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "orv_r_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn orv_sve_validates() { validate_instruction(&orv_sve_instruction()).expect("ORV SVE should validate"); }