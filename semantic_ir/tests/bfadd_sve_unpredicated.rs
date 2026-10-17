/// BFADD (unpredicated) - BFloat16 add (unpredicated)
/// Stanford source: arm64_md_clean/bfadd_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn bfadd_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W16;

    // Read first source vector Z0
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };

    // Read second source vector Z1
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };

    // Read destination vector Z0 (for merging predication - not used in unpredicated)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Bfloat16Add,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // No predicate - all active
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // Unpredicated = zeroing
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFADD".to_string(),
            encoding: Some("BFloat16 add (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_B16B16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfadd_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfadd_sve_unpredicated_validates() {
    let instruction = bfadd_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("BFADD SVE unpredicated should validate");
}

#[test]
fn bfadd_sve_unpredicated_uses_scalable_vector_map2() {
    let instruction = bfadd_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for BFADD SVE unpredicated");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Bfloat16Add);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W16);
    assert!(matches!(predicate.as_ref(), Expr::Const { value: 0, ty: BitWidth::W32 }));
    assert!(matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
    assert!(matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
}

#[test]
fn bfadd_sve_unpredicated_element_count() {
    let instruction = bfadd_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 16);
}