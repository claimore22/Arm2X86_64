/// BIC (vectors, unpredicated) - Bitwise clear (unpredicated)
/// Stanford source: arm64_md_clean/bic_z_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let operand1 : bits(VL) = Z(n);
///   let operand2 : bits(VL) = Z(m);
///   Z(VL)(d) = operand1 AND (NOT operand2);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn bic_sve_unpredicated_instruction() -> Instruction {
    // Using VL=256, esize=32 (8 elements)
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    // Read first source vector Z1
    let operand1 = Expr::ReadScalableVecReg {
        reg: 1,
        vl,
    };

    // Read second source vector Z2
    let operand2 = Expr::ReadScalableVecReg {
        reg: 2,
        vl,
    };

    // Read destination vector Z0
    let destination = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };

    // ScalableVectorMap2 with SVEVectorBinOp::Bic (unpredicated - no predicate)
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Bic,
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
            mnemonic: "BIC".to_string(),
            encoding: Some("Bitwise clear (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bic_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bic_sve_unpredicated_validates() {
    let instruction = bic_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("BIC SVE unpredicated should validate");
}

#[test]
fn bic_sve_unpredicated_uses_scalable_vector_map2() {
    let instruction = bic_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, predicate: _, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for BIC SVE unpredicated");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Bic);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    assert!(matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 }));
    assert!(matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
}

#[test]
fn bic_sve_unpredicated_element_count() {
    let instruction = bic_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    // VL=256, esize=32 -> 8 elements
    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 8);
}