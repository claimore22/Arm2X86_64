/// FLOGB (SVE predicated) - Floating-point base 2 logarithm as integer
/// Stanford source: arm64_md_clean/flogb_z_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   let operand : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](n) else Zeros(VL);
///   var result : bits(VL) = if merging then Z[](d) else Zeros(VL);
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let element : bits(esize) = operand[e*:esize];
///           result[e*esize] = FPLogB(esize)(element, FPCR());
///       end;
///   end;
///   Z[](d) = result;
///
/// Returns the unbiased exponent (base-2 logarithm) of each floating-point element as an integer.
/// - Normal x: exponent of x
/// - Subnormal: exponent of normalized representation
/// - Infinite: 2^(esize-1)-1 (max positive integer of width esize)
/// - ±0.0 or NaN: -2^(esize-1) (min negative integer of width esize)
/// Both merging and zeroing predication variants.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn flogb_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Flogb,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination), vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FLOGB".to_string(),
            encoding: Some("Floating-point base 2 logarithm as integer (predicated, merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "flogb_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn flogb_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Flogb,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FLOGB".to_string(),
            encoding: Some("Floating-point base 2 logarithm as integer (predicated, zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "flogb_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn flogb_sve_merging_validates() {
    let instruction = flogb_sve_merging_instruction();
    validate_instruction(&instruction).expect("FLOGB SVE merging should validate");
}

#[test]
fn flogb_sve_merging_structure() {
    let instruction = flogb_sve_merging_instruction();

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
        panic!("expected ScalableVectorMap for FLOGB merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Flogb);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn flogb_sve_zeroing_validates() {
    let instruction = flogb_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FLOGB SVE zeroing should validate");
}

#[test]
fn flogb_sve_zeroing_structure() {
    let instruction = flogb_sve_zeroing_instruction();

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
        panic!("expected ScalableVectorMap for FLOGB zeroing");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Flogb);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}