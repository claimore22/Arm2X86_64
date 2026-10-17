/// FTSSEL (SVE unpredicated) - Floating-point trigonometric select coefficient
/// Stanford source: arm64_md_clean/ftssel_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ftssel_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // FTSSEL: trigonometric select coefficient
    // Selects either element of first source or 1.0 based on bit<0> of second source
    // Negates if bit<1> of second source is set
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Ftssel,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand1),   // Zn
        right: Box::new(operand2),  // Zm
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // Unpredicated
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FTSSEL".to_string(),
            encoding: Some("Floating-point trigonometric select coefficient".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ftssel_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ftssel_sve_validates() {
    let instruction = ftssel_sve_instruction();
    validate_instruction(&instruction).expect("FTSSEL SVE should validate");
}

#[test]
fn ftssel_sve_structure() {
    let instruction = ftssel_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 { op, left, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap2 for FTSSEL");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Ftssel);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}