/// UUNPKHI - Unsigned unpack and extend high half of vector
/// Stanford source: arm64_md_clean/uunpkhi_z_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn uunpkhi_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Result element size (doubled)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    // UUNPKHI: unpack high half, zero-extend to double width
    // This is modeled as a special operation with an immediate indicating hi=true
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Uunpkhi,
        predicate: None,
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: Some(1), // hi = true for high half
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "UUNPKHI".to_string(),
            encoding: Some("Unsigned unpack and extend high half of vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uunpkhi_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// UUNPKLO - Unsigned unpack and extend low half of vector
fn uunpklo_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Result element size (doubled)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Uunpklo,
        predicate: None,
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: Some(0), // hi = false for low half
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "UUNPKLO".to_string(),
            encoding: Some("Unsigned unpack and extend low half of vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uunpkhi_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uunpkhi_sve_validates() {
    let instruction = uunpkhi_sve_instruction();
    validate_instruction(&instruction).expect("UUNPKHI SVE should validate");
}

#[test]
fn uunpklo_sve_validates() {
    let instruction = uunpklo_sve_instruction();
    validate_instruction(&instruction).expect("UUNPKLO SVE should validate");
}

#[test]
fn uunpkhi_sve_structure() {
    let instruction = uunpkhi_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for UUNPKHI");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Uunpkhi);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(*imm, Some(1));
}

#[test]
fn uunpklo_sve_structure() {
    let instruction = uunpklo_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for UUNPKLO");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Uunpklo);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(*imm, Some(0));
}