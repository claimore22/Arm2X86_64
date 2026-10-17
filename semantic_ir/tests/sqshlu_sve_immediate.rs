/// SQSHLU (immediate) - Signed saturating shift left unsigned by immediate
/// Stanford source: arm64_md_clean/sqshlu_z_p_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn sqshlu_sve_immediate_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - source / destination
    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 }; // shift amount (0 to esize-1)

    // SQSHLU: signed saturating shift left unsigned by immediate
    // Result is saturated to unsigned range [0, 2^N-1]
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sqshlu,
        predicate: Box::new(predicate),
        left: Box::new(operand.clone()),      // Zdn - source
        right: Box::new(shift_amount), // immediate shift amount
        destination: Box::new(operand),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQSHLU (immediate)".to_string(),
            encoding: Some("Signed saturating shift left unsigned by immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqshlu_z_p_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqshlu_sve_immediate_validates() {
    let instruction = sqshlu_sve_immediate_instruction();
    validate_instruction(&instruction).expect("SQSHLU SVE immediate should validate");
}

#[test]
fn sqshlu_sve_immediate_structure() {
    let instruction = sqshlu_sve_immediate_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for SQSHLU immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sqshlu);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::Const { value: 4, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}