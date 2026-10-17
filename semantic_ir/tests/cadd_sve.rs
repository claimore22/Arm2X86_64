/// CADD (SVE unpredicated) - Complex integer add
/// Stanford source: arm64_md_clean/cadd_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn cadd_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - first source / destination
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zm - second source

    // CADD: complex integer add with rotation
    // rot = 0: multiply second source by +j (sub_i=true, sub_r=false)
    // rot = 1: multiply second source by -j (sub_i=false, sub_r=true)
    let rotation = Expr::Const { value: 0, ty: BitWidth::W1 }; // rot=0

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Cadd,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2), Box::new(rotation)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "CADD".to_string(),
            encoding: Some("Complex integer add (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cadd_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cadd_sve_validates() {
    let instruction = cadd_sve_instruction();
    validate_instruction(&instruction).expect("CADD SVE should validate");
}

#[test]
fn cadd_sve_structure() {
    let instruction = cadd_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for CADD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Cadd);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 3);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[2].as_ref(), Expr::Const { value: 0, ty: BitWidth::W1 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W32);
    assert!(imm.is_none());
}