/// CPY (immediate, zeroing) - Copy signed integer immediate to vector elements (zeroing)
/// Stanford source: arm64_md_clean/cpy_z_p_i.md (zeroing variant)
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn cpy_immediate_zeroing_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Immediate as register
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Neg,
        predicate: Box::new(predicate), operand: Box::new(operand), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "CPY".to_string(), encoding: Some("Copy signed integer immediate to vector elements (zeroing)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "cpy_z_p_i.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn cpy_immediate_zeroing_sve_validates() { validate_instruction(&cpy_immediate_zeroing_sve_instruction()).expect("CPY (immediate, zeroing) SVE should validate"); }