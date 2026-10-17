/// LD1SW (vector plus immediate) - Gather load signed words to vector (immediate index)
/// Stanford source: arm64_md_clean/ld1sw_z_p_ai.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn ld1sw_sve_gather_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - base vector
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    // LD1SW: gather load signed words (immediate index)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ld1swGatherImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(base), Box::new(Expr::Const { value: 0, ty: BitWidth::W6 })], // offset
        vl,
        esize: Some(esize),
        imm: Some(0), // offset
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1SW (vector plus immediate)".to_string(),
            encoding: Some("Gather load signed words to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sw_z_p_ai.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1sw_sve_gather_imm_validates() {
    let instruction = ld1sw_sve_gather_imm_instruction();
    validate_instruction(&instruction).expect("LD1SW gather imm SVE should validate");
}

#[test]
fn ld1sw_sve_gather_imm_structure() {
    let instruction = ld1sw_sve_gather_imm_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for LD1SW gather imm");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Ld1swGatherImm);
    matches!(&**predicate.as_ref().unwrap(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::Const { value: 0, ty: BitWidth::W6 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W64);
    assert!(imm.is_some());
}