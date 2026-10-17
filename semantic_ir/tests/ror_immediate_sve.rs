/// ROR (immediate) - Rotate right by immediate
/// Stanford source: arm64_md_clean/ror_z_zi.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn ror_immediate_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let shift_amount = Expr::ReadScalableVecReg { reg: 1, vl };  // Shift amount as vector
    let operand = Expr::ReadScalableVecReg { reg: 0, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Lsr,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(operand), right: Box::new(shift_amount), destination: Box::new(destination),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "ROR".to_string(), encoding: Some("Rotate right by immediate".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "ror_z_zi.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}
#[test]
fn ror_immediate_sve_validates() { validate_instruction(&ror_immediate_sve_instruction()).expect("ROR SVE immediate should validate"); }