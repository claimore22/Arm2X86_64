/// SRSHL - Signed rounding shift left (predicated)
/// Stanford source: arm64_md_clean/srshl_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn srshl_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn - first source/destination
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - second source (shift amounts)

    // SRSHL: signed rounding shift left (predicated)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Srshl,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 1, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SRSHL".to_string(),
            encoding: Some("Signed rounding shift left (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "srshl_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn srshl_sve_validates() {
    let instruction = srshl_sve_instruction();
    validate_instruction(&instruction).expect("SRSHL SVE should validate");
}

#[test]
fn srshl_sve_structure() {
    let instruction = srshl_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for SRSHL");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Srshl);
    matches!(&**predicate.as_ref().unwrap(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W64);
    assert!(imm.is_none());
}