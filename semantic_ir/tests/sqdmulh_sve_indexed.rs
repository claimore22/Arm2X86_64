/// SQDMULH (indexed, unpredicated) - Signed saturating doubling multiply high by indexed element
/// Stanford source: arm64_md_clean/sqdmulh_z_zzi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sqdmulh_sve_indexed_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // SQDMULH indexed: multiply by indexed element from Zm
    // Using same pattern as FMUL indexed: index in left (as vector register)
    let index = Expr::ReadScalableVecReg { reg: 3, vl }; // Index register

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Sqdmulh,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(index),       // Index register
        middle: Box::new(operand1),  // First source Zn
        right: Box::new(operand2),   // Second source Zm
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQDMULH (indexed)".to_string(),
            encoding: Some("Signed saturating doubling multiply high by indexed element".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqdmulh_z_zzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqdmulh_sve_indexed_validates() {
    let instruction = sqdmulh_sve_indexed_instruction();
    validate_instruction(&instruction).expect("SQDMULH SVE indexed should validate");
}

#[test]
fn sqdmulh_sve_indexed_structure() {
    let instruction = sqdmulh_sve_indexed_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for SQDMULH indexed");
    };
    // Note: We're using Sqdmulh as a ternary op for indexed version
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 3, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}