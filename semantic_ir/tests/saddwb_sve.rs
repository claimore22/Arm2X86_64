/// SADDWB - Signed add wide (bottom)
/// Stanford source: arm64_md_clean/saddwb_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn saddwb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;  // SADDWB produces 64-bit results

    // SADDWB adds the bottom 32-bit elements (sign-extended) to 64-bit elements
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // Unpredicated
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(Expr::Const { value: 0, ty: BitWidth::W256 }), // Unpredicated
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SADDWB".to_string(),
            encoding: Some("Signed add wide (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "saddwb_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn saddwb_sve_validates() {
    let instruction = saddwb_sve_instruction();
    validate_instruction(&instruction).expect("SADDWB SVE should validate");
}