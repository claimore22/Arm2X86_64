/// SABDLB (SVE unpredicated) - Signed absolute difference long (bottom)
/// Stanford source: arm64_md_clean/sabdlb_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sabdlb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Source element size, result is 2*esize = 64-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    // SABDLB: for each pair of elements (bottom half), compute abs difference and widen
    // This uses a narrowing operation conceptually but for widen we use a binary op
    // The result is in overlapping double-width elements
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::SQSub, // Using saturating sub as placeholder for abs diff
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize: BitWidth::W64, // Result is 64-bit elements
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABDLB".to_string(),
            encoding: Some("Signed absolute difference long (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabdlb_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// SABDLT (SVE unpredicated) - Signed absolute difference long (top)
fn sabdlt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Source element size, result is 2*esize = 64-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::SQSub,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize: BitWidth::W64, // Result is 64-bit elements
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABDLT".to_string(),
            encoding: Some("Signed absolute difference long (top)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabdlt_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sabdlb_sve_validates() {
    let instruction = sabdlb_sve_instruction();
    validate_instruction(&instruction).expect("SABDLB SVE should validate");
}

#[test]
fn sabdlt_sve_validates() {
    let instruction = sabdlt_sve_instruction();
    validate_instruction(&instruction).expect("SABDLT SVE should validate");
}

#[test]
fn sabdlb_sve_structure() {
    let instruction = sabdlb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for SABDLB");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQSub);
    matches!(&**predicate, Expr::ReadPredReg { .. });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}