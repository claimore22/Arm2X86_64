/// FMLA, FMLS (SVE predicated) - Floating-point fused multiply-add/subtract
/// Stanford sources: arm64_md_clean/fmla_z_p_zzz.md, arm64_md_clean/fmls_z_p_zzz.md
///
/// FMLA (vectors):
///   Zda = Zda + Zn * Zm (active), Zda (inactive) - destructive on Zda
///   FMLS (vectors):
///   Zda = Zda - Zn * Zm (active), Zda (inactive) - destructive on Zda
///
/// Both use the same FPMulAdd operation, with negated first operand for FMLS.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fmla_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let addend = Expr::ReadScalableVecReg { reg: 0, vl };   // Zda (addend/destination)

    // FMLA: Zda = Zda + Zn * Zm
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmla,
        predicate: Box::new(predicate),
        left: Box::new(operand1),         // Zn
        middle: Box::new(operand2),       // Zm
        right: Box::new(addend.clone()),  // Zda (addend)
        destination: Box::new(addend),    // Zda (destination)
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zda
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMLA".to_string(),
            encoding: Some("Floating-point fused multiply-add (vectors, predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmla_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fmls_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let addend = Expr::ReadScalableVecReg { reg: 0, vl };   // Zda (addend/destination)

    // FMLS: Zda = Zda - Zn * Zm (uses Fmls ternary op, which is FPMulAdd with negated first operand)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmls,
        predicate: Box::new(predicate),
        left: Box::new(operand1),         // Zn
        middle: Box::new(operand2),       // Zm
        right: Box::new(addend.clone()),  // Zda (addend)
        destination: Box::new(addend),    // Zda (destination)
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Zda
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMLS".to_string(),
            encoding: Some("Floating-point fused multiply-subtract (vectors, predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmls_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmla_sve_predicated_validates() {
    let instruction = fmla_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMLA SVE predicated should validate");
}

#[test]
fn fmla_sve_predicated_structure() {
    let instruction = fmla_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 {
        op,
        predicate,
        left,
        middle,
        right,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap3 for FMLA");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fmla);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fmls_sve_predicated_validates() {
    let instruction = fmls_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMLS SVE predicated should validate");
}

#[test]
fn fmls_sve_predicated_structure() {
    let instruction = fmls_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 {
        op,
        predicate,
        left,
        middle,
        right,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap3 for FMLS");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fmls);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}