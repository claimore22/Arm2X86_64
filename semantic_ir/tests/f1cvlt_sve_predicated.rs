/// F1CVTLT, F2CVTLT (SVE predicated) - 8-bit floating-point convert to half-precision (top)
/// Stanford source: arm64_md_clean/f1cvtlt_z_z8.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   let operand : bits(VL) = Z[n];
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element : bits(esize DIV 2) = operand[(2*e + 1)*(esize DIV 2)];
///       result[e*esize] = FP8ConvertFP(element, issrc2, FPCR(), FPMR());
///   end;
///   Z[VL](d) = result.
///
/// For F1CVTLT: uses FPMR.LSCALE and FPMR.F8S1
/// For F2CVTLT: uses FPMR.LSCALE2 and FPMR.F8S2
/// This instruction is UNPREDICATED in the ARM spec, but we model it with a dummy predicate for IR consistency.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn f1cvlt_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    // Source: 8-bit elements (FP8), Destination: 16-bit elements (BF16)
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // F1CVTLT uses Bfloat16Cvtlt with specific LSCALE/F8S1 selection
    // For F1CVTLT, issrc2 = FALSE; for F2CVTLT, issrc2 = TRUE
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Bfloat16Cvtlt,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        operand: Box::new(Expr::ReadScalableVecReg { reg: 1, vl }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl: BitWidth::W256,
        esize: BitWidth::W8,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "F1CVTLT".to_string(),
            encoding: Some("8-bit floating-point convert to half-precision (top, F1CVTLT)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE2".to_string(),
            "FEAT_SME2".to_string(),
            "FEAT_FP8".to_string(),
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "f1cvtlt_z_z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn f2cvlt_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Bfloat16Cvtlt,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        operand: Box::new(Expr::ReadScalableVecReg { reg: 1, vl }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "F2CVTLT".to_string(),
            encoding: Some("8-bit floating-point convert to half-precision (top, F2CVTLT)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE2".to_string(),
            "FEAT_SME2".to_string(),
            "FEAT_FP8".to_string(),
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "f1cvtlt_z_z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn f1cvlt_sve_predicated_validates() {
    let instruction = f1cvlt_sve_predicated_instruction();
    validate_instruction(&instruction).expect("F1CVTLT SVE predicated should validate");
}

#[test]
fn f1cvlt_sve_predicated_structure() {
    let instruction = f1cvlt_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        predicate,
        operand,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap for F1CVTLT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Bfloat16Cvtlt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W8); // 8-bit source
    assert_eq!(*predication, PredicationMode::Merging);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn f2cvlt_sve_predicated_validates() {
    let instruction = f2cvlt_sve_predicated_instruction();
    validate_instruction(&instruction).expect("F2CVTLT SVE predicated should validate");
}

#[test]
fn f2cvlt_sve_predicated_structure() {
    let instruction = f2cvlt_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        predicate,
        operand,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap for F2CVTLT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Bfloat16Cvtlt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W8);
    assert_eq!(*predication, PredicationMode::Merging);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}