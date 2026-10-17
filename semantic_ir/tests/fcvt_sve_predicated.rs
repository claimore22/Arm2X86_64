/// FCVT (SVE predicated) - Floating-point convert
/// Stanford source: arm64_md_clean/fcvt_z_p_z.md
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
///           let res : bits(d_esize) = FPConvertSVE(d_esize, s_esize)(element[s_esize-1:0], FPCR());
///           result[e*esize] = ZeroExtend(esize)(res);
///       end;
///   end;
///   Z[](d) = result.
///
/// Converts between floating-point precisions (FP16 <-> FP32 <-> FP64).
/// Source elements are s_esize bits, destination elements are d_esize bits.
/// The smaller type is held unpacked in the least significant bits of the larger size.
/// When result is smaller, results are zero-extended to fill each destination element.
/// This instruction has both merging and zeroing predication variants.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcvt_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // esize = 64 (destination element size)
    let s_esize = BitWidth::W32; // s_esize = 32 (source element size)

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[d] - destination

    // FCVT: convert from single-precision (32-bit) to double-precision (64-bit)
    // Using SVEVectorUnaryOp::Fcvt with source element size 32, destination element size 64
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fcvt,
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
            mnemonic: "FCVT".to_string(),
            encoding: Some("Floating-point convert single to double (predicated, merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvt_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcvt_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let s_esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fcvt,
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
            mnemonic: "FCVT".to_string(),
            encoding: Some("Floating-point convert single to double (predicated, zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvt_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvt_sve_merging_validates() {
    let instruction = fcvt_sve_merging_instruction();
    validate_instruction(&instruction).expect("FCVT SVE merging should validate");
}

#[test]
fn fcvt_sve_merging_structure() {
    let instruction = fcvt_sve_merging_instruction();

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
        panic!("expected ScalableVectorMap for FCVT merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Fcvt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64); // Destination element size (64-bit)
    assert_eq!(*predication, PredicationMode::Merging);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}

#[test]
fn fcvt_sve_zeroing_validates() {
    let instruction = fcvt_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FCVT SVE zeroing should validate");
}

#[test]
fn fcvt_sve_zeroing_structure() {
    let instruction = fcvt_sve_zeroing_instruction();

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
        panic!("expected ScalableVectorMap for FCVT zeroing");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Fcvt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, PredicationMode::Zeroing);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}