/// UZPQ1 - Concatenate even elements within each pair of quadword vector segments
/// Stanford source: arm64_md_clean/uzpq1_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn uzpq1_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Uzp1, // Using Uzp1 as placeholder
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(esize),
        imm: Some(0), // part = 0 for UZPQ1
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "UZPQ1".to_string(),
            encoding: Some("Concatenate even elements within each pair of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uzpq1_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// UZPQ2 - Concatenate odd elements within each pair of quadword vector segments
fn uzpq2_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Uzp1, // Using Uzp1 as placeholder
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(esize),
        imm: Some(1), // part = 1 for UZPQ2
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "UZPQ2".to_string(),
            encoding: Some("Concatenate odd elements within each pair of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uzpq2_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uzpq1_sve_validates() {
    let instruction = uzpq1_sve_instruction();
    validate_instruction(&instruction).expect("UZPQ1 SVE should validate");
}

#[test]
fn uzpq2_sve_validates() {
    let instruction = uzpq2_sve_instruction();
    validate_instruction(&instruction).expect("UZPQ2 SVE should validate");
}

#[test]
fn uzpq1_sve_structure() {
    let instruction = uzpq1_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for UZPQ1");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Uzp1);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 2);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(*imm, Some(0));
}

#[test]
fn uzpq2_sve_structure() {
    let instruction = uzpq2_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for UZPQ2");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Uzp1);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 2);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(*imm, Some(1));
}