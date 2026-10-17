/// CDOT (vectors, unpredicated) - Complex integer dot product
/// Stanford source: arm64_md_clean/cdot_z_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn cdot_vectors_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - first source
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zda - addend/destination

    // CDOT: complex integer dot product
    // Uses rotation 0 for this test (real part accumulation)
    let rotation = Expr::Const { value: 0, ty: BitWidth::W2 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Cdot,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2), Box::new(rotation)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "CDOT (vectors)".to_string(),
            encoding: Some("Complex integer dot product".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cdot_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cdot_vectors_sve_validates() {
    let instruction = cdot_vectors_sve_instruction();
    validate_instruction(&instruction).expect("CDOT vectors SVE should validate");
}

#[test]
fn cdot_vectors_sve_structure() {
    let instruction = cdot_vectors_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for CDOT vectors");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Cdot);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 3);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(operands[2].as_ref(), Expr::Const { value: 0, ty: BitWidth::W2 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W64);
    assert!(imm.is_none());
}