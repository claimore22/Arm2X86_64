/// EXPAND (SVE) - Copy lower-numbered vector elements to Active elements
/// Stanford source: arm64_md_clean/expand_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn expand_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - source

    // EXPAND: copy consecutive elements from source to active elements of destination
    // Inactive elements are set to zero
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Expand,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "EXPAND".to_string(),
            encoding: Some("Copy lower-numbered vector elements to Active elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "expand_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn expand_sve_validates() {
    let instruction = expand_sve_instruction();
    validate_instruction(&instruction).expect("EXPAND SVE should validate");
}

#[test]
fn expand_sve_structure() {
    let instruction = expand_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for EXPAND");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Expand);
    matches!(&**predicate.as_ref().unwrap(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W32);
    assert!(imm.is_none());
}