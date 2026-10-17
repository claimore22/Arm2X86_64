/// SQRDMLSH (vectors, unpredicated) - Signed saturating rounding doubling multiply-subtract high
/// Stanford source: arm64_md_clean/sqrdmlsh_z_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sqrdmlsh_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zda - addend/destination

    // SQRDMLSH: signed saturating rounding doubling multiply-subtract high (unpredicated)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Sqrdmlsh,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),      // Zn
        middle: Box::new(operand2),    // Zm
        right: Box::new(destination.clone()),  // Zda (addend)
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQRDMLSH (vectors)".to_string(),
            encoding: Some("Signed saturating rounding doubling multiply-subtract high (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqrdmlsh_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqrdmlsh_sve_validates() {
    let instruction = sqrdmlsh_sve_instruction();
    validate_instruction(&instruction).expect("SQRDMLSH SVE should validate");
}

#[test]
fn sqrdmlsh_sve_structure() {
    let instruction = sqrdmlsh_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for SQRDMLSH");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Sqrdmlsh);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}