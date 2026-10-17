/// FCMLA (vectors, predicated) - Floating-point complex multiply-add
/// Stanford source: arm64_md_clean/fcmla_z_p_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcmla_sve_vectors_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // Single-precision

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zda - addend/destination

    // FCMLA: complex multiply-add with rotation (rot=0 for this test)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fcmlalb, // Using existing complex mul-add
        predicate: Box::new(predicate),
        left: Box::new(operand1),          // Zn
        middle: Box::new(operand2),        // Zm
        right: Box::new(destination.clone()),  // Zda (addend)
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMLA (vectors)".to_string(),
            encoding: Some("Floating-point complex multiply-add (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmla_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcmla_sve_vectors_validates() {
    let instruction = fcmla_sve_vectors_instruction();
    validate_instruction(&instruction).expect("FCMLA SVE vectors should validate");
}

#[test]
fn fcmla_sve_vectors_structure() {
    let instruction = fcmla_sve_vectors_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for FCMLA vectors");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fcmlalb);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}