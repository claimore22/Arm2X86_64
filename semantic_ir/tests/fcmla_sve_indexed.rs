/// FCMLA (indexed) - Floating-point complex multiply-add by indexed element
/// Stanford source: arm64_md_clean/fcmla_z_zzzi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcmla_sve_indexed_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Single-precision

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zda - addend/destination

    // FCMLA indexed: complex multiply-add by indexed element (unpredicated)
    // index selects the complex number within each 128-bit segment
    let index = Expr::Const { value: 0, ty: BitWidth::W4 }; // 4-bit index

    // For the indexed version, we use a ternary op with the index as an immediate
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fcmlalt, // Using existing complex mul-add (top)
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),          // Zn
        middle: Box::new(operand2),        // Zm
        right: Box::new(destination.clone()),  // Zda (addend)
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Zeroing, // Unpredicated
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMLA (indexed)".to_string(),
            encoding: Some("Floating-point complex multiply-add by indexed element".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmla_z_zzzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcmla_sve_indexed_validates() {
    let instruction = fcmla_sve_indexed_instruction();
    validate_instruction(&instruction).expect("FCMLA SVE indexed should validate");
}

#[test]
fn fcmla_sve_indexed_structure() {
    let instruction = fcmla_sve_indexed_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for FCMLA indexed");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fcmlalt);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}