/// LDNT1B (scalar plus immediate, single register) - Contiguous load non-temporal bytes
/// Stanford source: arm64_md_clean/ldnt1b_z_p_bi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn ldnt1b_sve_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base register
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    // LDNT1B: contiguous load non-temporal bytes (immediate index)
    let offset = Expr::Const { value: 0, ty: BitWidth::W4 }; // offset

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ldnt1bImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(base), Box::new(offset)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDNT1B (scalar plus immediate)".to_string(),
            encoding: Some("Contiguous load non-temporal bytes to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnt1b_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ldnt1b_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let offset = Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 }; // Rm - scalar index
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    // LDNT1B: contiguous load non-temporal bytes (scalar index)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ldnt1bScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(base), Box::new(offset)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDNT1B (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous load non-temporal bytes to vector (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnt1b_z_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnt1b_sve_imm_validates() {
    let instruction = ldnt1b_sve_imm_instruction();
    validate_instruction(&instruction).expect("LDNT1B imm SVE should validate");
}

#[test]
fn ldnt1b_sve_scalar_validates() {
    let instruction = ldnt1b_sve_scalar_instruction();
    validate_instruction(&instruction).expect("LDNT1B scalar SVE should validate");
}

#[test]
fn ldnt1b_sve_imm_structure() {
    let instruction = ldnt1b_sve_imm_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for LDNT1B imm");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Ldnt1bImm);
    matches!(&**predicate.as_ref().unwrap(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 });
    matches!(operands[1].as_ref(), Expr::Const { value: 0, ty: BitWidth::W4 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W8);
    assert!(imm.is_none());
}

#[test]
fn ldnt1b_sve_scalar_structure() {
    let instruction = ldnt1b_sve_scalar_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for LDNT1B scalar");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Ldnt1bScalar);
    matches!(&**predicate.as_ref().unwrap(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 });
    matches!(operands[1].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W8);
    assert!(imm.is_none());
}