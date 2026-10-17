/// SUB (immediate) - Subtract immediate (unpredicated)
/// Stanford source: arm64_md_clean/sub_z_zi.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn sub_immediate_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let immediate = Expr::ReadScalableVecReg { reg: 2, vl };  // Immediate as vector
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sub,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // No predicate
        left: Box::new(operand), right: Box::new(immediate), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "SUB".to_string(), encoding: Some("Subtract immediate (unpredicated)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "sub_z_zi.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn sub_immediate_sve_validates() { validate_instruction(&sub_immediate_sve_instruction()).expect("SUB immediate SVE should validate"); }