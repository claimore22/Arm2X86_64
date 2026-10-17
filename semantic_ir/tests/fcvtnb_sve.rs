/// FCVTNB (unpredicated) - Single-precision convert to interleaved 8-bit floating-point (bottom)
/// Stanford source: arm64_md_clean/fcvtnb_z8_mz2.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcvtnb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Z[n+1] - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[d] - destination

    // FCVTNB: convert two single-precision vectors to interleaved 8-bit FP (bottom)
    let predicate = Expr::Const { value: 0, ty: BitWidth::W32 }; // dummy predicate for unpredicated
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fcvtnb,
        predicate: Box::new(predicate),
        left: Box::new(operand1),  // Z[n]
        middle: Box::new(operand2), // Z[n+1]
        right: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }), // destination
        destination: Box::new(destination),
        vl,
        esize: BitWidth::W16, // 16-bit container for two 8-bit results
        predication: PredicationMode::Zeroing, // Unpredicated uses Zeroing
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCVTNB".to_string(),
            encoding: Some("Single-precision convert to interleaved 8-bit floating-point (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_FP8".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtnb_z8_mz2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtnb_sve_validates() {
    let instruction = fcvtnb_sve_instruction();
    validate_instruction(&instruction).expect("FCVTNB SVE should validate");
}

#[test]
fn fcvtnb_sve_structure() {
    let instruction = fcvtnb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for FCVTNB");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fcvtnb);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W16);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}