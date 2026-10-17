/// URSHR (predicated) - Unsigned rounding shift right by immediate
/// Stanford source: arm64_md_clean/urshr_z_p_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn urshr_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - source / destination

    // URSHR: unsigned rounding shift right by immediate (merging)
    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 }; // shift amount

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Ursh,
        predicate: Box::new(predicate),
        left: Box::new(operand.clone()),      // Zdn - source
        middle: Box::new(shift_amount), // immediate shift amount
        right: Box::new(operand.clone()), // Zdn - destination
        destination: Box::new(operand),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "URSHR (merging)".to_string(),
            encoding: Some("Unsigned rounding shift right by immediate (merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "urshr_z_p_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn urshr_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 0, vl };

    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 };

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Ursh,
        predicate: Box::new(predicate),
        left: Box::new(operand.clone()),
        middle: Box::new(shift_amount),
        right: Box::new(operand.clone()),
        destination: Box::new(operand),
        vl,
        esize,
        predication: PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "URSHR (zeroing)".to_string(),
            encoding: Some("Unsigned rounding shift right by immediate (zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "urshr_z_p_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn urshr_sve_merging_validates() {
    let instruction = urshr_sve_merging_instruction();
    validate_instruction(&instruction).expect("URSHR SVE merging should validate");
}

#[test]
fn urshr_sve_zeroing_validates() {
    let instruction = urshr_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("URSHR SVE zeroing should validate");
}

#[test]
fn urshr_sve_merging_structure() {
    let instruction = urshr_sve_merging_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for URSHR merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Ursh);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}