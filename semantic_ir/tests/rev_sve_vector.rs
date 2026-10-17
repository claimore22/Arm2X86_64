/// REV (vector) - Reverse all elements in a vector (unpredicated)
/// Stanford source: arm64_md_clean/rev_z_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn rev_sve_vector_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Rev,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        operand: Box::new(operand),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "REV".to_string(),
            encoding: Some("Reverse all elements in a vector (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rev_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rev_sve_vector_validates() {
    let instruction = rev_sve_vector_instruction();
    validate_instruction(&instruction).expect("REV SVE vector should validate");
}