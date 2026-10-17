/// FMLALT (indexed, FP16 to FP32) - Floating-point multiply-add long (top)
/// Stanford source: arm64_md_clean/fmlalt_z_z8z8z8.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fmlalt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - FP16 elements
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - FP16 elements
    let addend = Expr::ReadScalableVecReg { reg: 3, vl };   // Zda - FP32 elements

    // FMLALT: Zda = Zda + Zn * Zm (top half elements, widened)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fcmlalt,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        middle: Box::new(operand2),
        right: Box::new(addend.clone()),
        destination: Box::new(addend),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 3,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMLALT (indexed, FP16 to FP32)".to_string(),
            encoding: Some("Floating-point multiply-add long (top)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmlalt_z_z8z8z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmlalt_sve_indexed_validates() {
    let instruction = fmlalt_sve_instruction();
    validate_instruction(&instruction).expect("FMLALT (indexed, FP16 to FP32) should validate");
}
