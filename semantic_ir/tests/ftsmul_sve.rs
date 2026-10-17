/// FTSMUL - Floating-point trigonometric starting value (unpredicated)
/// Stanford source: arm64_md_clean/ftsmul_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ftsmul_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    // Read first source vector Z0
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };

    // Read second source vector Z1
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };

    // Read destination vector Z0
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Ftsmul,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // No predicate - unpredicated
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FTSMUL".to_string(),
            encoding: Some("Floating-point trigonometric starting value".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ftsmul_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ftsmul_sve_validates() {
    let instruction = ftsmul_sve_instruction();
    validate_instruction(&instruction).expect("FTSMUL SVE should validate");
}