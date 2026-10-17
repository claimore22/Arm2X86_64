/// MATCH, NMATCH (SVE2) - Detect matching/non-matching elements within 128-bit segments
/// Stanford sources: arm64_md_clean/match_p_p_zz.md, arm64_md_clean/nmatch_p_p_zz.md
///
/// MATCH: For each active element in Zn, check if it matches any element in the corresponding
/// 128-bit segment of Zm. Set Pd true if match found, false otherwise.
///
/// NMATCH: For each active element in Zn, check if NO match is found in the corresponding
/// 128-bit segment of Zm. Set Pd true if NO match found, false otherwise.
///
/// Both:
/// - Operate on 8-bit or 16-bit elements (esize = 8 or 16)
/// - Compare within 128-bit segments (eltspersegment = 128/esize)
/// - Produce predicate result in Pd
/// - Set condition flags (N, Z, C, V) based on predicate result
/// - Inactive elements in Pd set to zero
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn match_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8; // esize = 8 or 16

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    // MATCH: check if any element in 128-bit segment matches
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Match,
        predicate: Some(Box::new(predicate)),
        operands: vec![
            Box::new(operand1),
            Box::new(operand2),
        ],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "MATCH".to_string(),
            encoding: Some("Detect any matching elements within 128-bit segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "match_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn nmatch_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    // NMATCH: check if NO element in 128-bit segment matches
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Nmatch,
        predicate: Some(Box::new(predicate)),
        operands: vec![
            Box::new(operand1),
            Box::new(operand2),
        ],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "NMATCH".to_string(),
            encoding: Some("Detect no matching elements within 128-bit segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "nmatch_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn match_sve_validates() {
    let instruction = match_sve_instruction();
    validate_instruction(&instruction).expect("MATCH SVE should validate");
}

#[test]
fn match_sve_structure() {
    let instruction = match_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for MATCH");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Match);
    let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
    let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W8));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}

#[test]
fn nmatch_sve_validates() {
    let instruction = nmatch_sve_instruction();
    validate_instruction(&instruction).expect("NMATCH SVE should validate");
}

#[test]
fn nmatch_sve_structure() {
    let instruction = nmatch_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for NMATCH");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Nmatch);
    let Some(pred) = &predicate else { panic!("expected predicate to be Some"); };
    let Expr::ReadPredReg { .. } = pred.as_ref() else { panic!("expected ReadPredReg for predicate"); };
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W8));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}