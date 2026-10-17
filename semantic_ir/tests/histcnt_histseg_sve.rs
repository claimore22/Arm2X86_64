/// HISTCNT, HISTSEG (SVE) - Count matching elements
/// Stanford sources: arm64_md_clean/histcnt_z_p_zz.md, arm64_md_clean/histseg_z_zz.md
///
/// HISTCNT (predicated):
///   For each active element e in Zn, count matches with Zm[0..e] (active elements only)
///   Result[e] = count of Zm[i] == Zn[e] for active i <= e
///   Inactive elements in Zd set to zero
///   esize = 32 or 64
///
/// HISTSEG (unpredicated):
///   For each 8-bit element in Zn, count matches within its 128-bit segment in Zm
///   Each 128-bit segment has 16 bytes (128/8)
///   Result[e] = count of Zm[i] == Zn[e] for i in same 128-bit segment
///   esize = 8
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn histcnt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 32 or 64

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    // HISTCNT: count matches of Zn[e] in Zm[0..e] for active elements
    // Using SVESpecialOp::Histcnt with predicate
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Histcnt,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "HISTCNT".to_string(),
            encoding: Some("Count matching elements in vector (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "histcnt_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn histseg_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W8; // esize = 8 (byte elements)

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    // HISTSEG: count matches of each byte in Zn within its 128-bit segment in Zm
    // Using SVESpecialOp::Histseg (unpredicated)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Histseg,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zd
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "HISTSEG".to_string(),
            encoding: Some("Count matching elements in vector segments (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "histseg_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn histcnt_sve_validates() {
    let instruction = histcnt_sve_instruction();
    validate_instruction(&instruction).expect("HISTCNT SVE should validate");
}

#[test]
fn histcnt_sve_structure() {
    let instruction = histcnt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for HISTCNT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Histcnt);
    assert!(predicate.is_some());
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}

#[test]
fn histseg_sve_validates() {
    let instruction = histseg_sve_instruction();
    validate_instruction(&instruction).expect("HISTSEG SVE should validate");
}

#[test]
fn histseg_sve_structure() {
    let instruction = histseg_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for HISTSEG");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Histseg);
    assert!(predicate.is_none()); // unpredicated
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W8));
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}