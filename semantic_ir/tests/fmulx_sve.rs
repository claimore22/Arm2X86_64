/// FMULX (SVE predicated) - Floating-point multiply extended
/// Stanford source: arm64_md_clean/fmulx_z_p_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   let operand1 : bits(VL) = Z[](dn);
///   let operand2 : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](m) else Zeros(VL);
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(esize) = operand1[e*:esize];
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let element2 : bits(esize) = operand2[e*:esize];
///           result[e*esize] = FPMulX(esize)(element1, element2, FPCR());
///       else
///           result[e*esize] = element1;
///       end;
///   end;
///   Z[](dn) = result;
///
/// Like FMUL but with special handling: ∞×0.0 gives 2.0 instead of NaN.
/// Used with FRECPX for safe vector normalization.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fmulx_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::FMulX,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 1, // Zdn
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMULX".to_string(),
            encoding: Some("Floating-point multiply extended (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmulx_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmulx_sve_predicated_validates() {
    let instruction = fmulx_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMULX SVE predicated should validate");
}

#[test]
fn fmulx_sve_predicated_structure() {
    let instruction = fmulx_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 {
        op,
        predicate,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap2 for FMULX");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::FMulX);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}