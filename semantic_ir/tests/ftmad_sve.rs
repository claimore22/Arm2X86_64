/// FTMAD (SVE unpredicated) - Floating-point trigonometric multiply-add coefficient
/// Stanford source: arm64_md_clean/ftmad_z_zzi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ftmad_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // FP64

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zm
    let imm_index = Expr::ReadScalableVecReg { reg: 2, vl }; // Immediate index register

    // FTMAD: Zdn = FPTrigMAdd(imm, Zdn, Zm)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Ftmad,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // Unpredicated
        left: Box::new(imm_index),       // Immediate index (modeled as vector register)
        middle: Box::new(operand1),      // Zdn
        right: Box::new(operand2),       // Zm
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FTMAD".to_string(),
            encoding: Some("Floating-point trigonometric multiply-add coefficient".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ftmad_z_zzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ftmad_sve_validates() {
    let instruction = ftmad_sve_instruction();
    validate_instruction(&instruction).expect("FTMAD SVE should validate");
}

#[test]
fn ftmad_sve_structure() {
    let instruction = ftmad_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for FTMAD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Ftmad);
    matches!(&**predicate, Expr::Const { value: 0, ty: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}