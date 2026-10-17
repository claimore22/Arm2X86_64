/// ORQV - Bitwise OR reduction of quadword vector segments
/// Stanford source: arm64_md_clean/orqv_z_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn orqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::OrQV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::V(0), value: result }];
    Instruction {
        id: InstructionId { mnemonic: "ORQV".to_string(), encoding: Some("Bitwise OR reduction of quadword vector segments".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2p1".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "orqv_z_p_z.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn orqv_sve_validates() { validate_instruction(&orqv_sve_instruction()).expect("ORQV SVE should validate"); }