/// FCVTLT (SVE predicated) - Floating-point widening convert (top)
/// Stanford source: arm64_md_clean/fcvtlt_z_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let halfesize : integer = esize DIV 2;
///   let mask : bits(PL) = P[](g);
///   let operand : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](n) else Zeros(VL);
///   var result : bits(VL) = if merging then Z[](d) else Zeros(VL);
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let element : bits(halfesize) = operand[(2*e + 1)*halfesize];
///           result[e*esize] = FPConvertSVE(esize, halfesize)(element, FPCR());
///       end;
///   end;
///   Z[](d) = result.
///
/// Converts odd-numbered half-precision elements to full-precision:
/// - Source: 2 elements per destination element (e.g., 2xFP16 -> 1xFP32, 2xFP32 -> 1xFP64)
/// - Only odd-numbered elements (index 1, 3, 5...) are converted
/// - Results placed in destination vector at full precision
/// This instruction has both merging and zeroing predication variants.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcvtlt_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // Destination element size (64-bit)
    let halfesize = BitWidth::W32; // Source element size (32-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - source (packed half-precision)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[d] - destination (full precision)

    // FCVTLT: convert odd-numbered 32-bit elements to 64-bit
    // Using SVEVectorUnaryOp::Fcvtlt
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fcvtlt,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize, // Destination element size (64-bit)
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCVTLT".to_string(),
            encoding: Some("Floating-point widening convert top single to double (predicated, merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtlt_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcvtlt_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let halfesize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fcvtlt,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCVTLT".to_string(),
            encoding: Some("Floating-point widening convert top single to double (predicated, zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtlt_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtlt_sve_merging_validates() {
    let instruction = fcvtlt_sve_merging_instruction();
    validate_instruction(&instruction).expect("FCVTLT SVE merging should validate");
}

#[test]
fn fcvtlt_sve_merging_structure() {
    let instruction = fcvtlt_sve_merging_instruction();

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
        panic!("expected ScalableVectorMap for FCVTLT merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Fcvtlt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64); // Destination element size (64-bit)
    assert_eq!(*predication, PredicationMode::Merging);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fcvtlt_sve_zeroing_validates() {
    let instruction = fcvtlt_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FCVTLT SVE zeroing should validate");
}

#[test]
fn fcvtlt_sve_zeroing_structure() {
    let instruction = fcvtlt_sve_zeroing_instruction();

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
        panic!("expected ScalableVectorMap for FCVTLT zeroing");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Fcvtlt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, PredicationMode::Zeroing);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}