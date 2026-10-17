/// FMAD (SVE predicated) - Floating-point fused multiply-add to multiplicand
/// Stanford source: arm64_md_clean/fmad_z_p_zzz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   let op1 : bits(VL) = Z[](dn);          // multiplicand (also destination)
///   let op2 : bits(VL) = Z[](m);            // multiplier
///   let op3 : bits(VL) = Z[](a);            // addend
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let elem1 : bits(esize) = op1[e*:esize];
///           let elem2 : bits(esize) = op2[e*:esize];
///           let elem3 : bits(esize) = op3[e*:esize];
///           result[e*esize] = FPMulAdd(esize)(elem3, elem1, elem2, FPCR());
///       else
///           result[e*esize] = op1[e*esize];
///       end;
///   end;
///   Z[](dn) = result;
///
/// FMAD: Zdn = Za + Zdn * Zm
/// - Zdn: multiplicand AND destination
/// - Zm: multiplier
/// - Za: addend
/// Different from FMLA where Zda is both addend and destination.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fmad_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let multiplicand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let multiplier = Expr::ReadScalableVecReg { reg: 2, vl };   // Zm
    let addend = Expr::ReadScalableVecReg { reg: 3, vl };       // Za

    // FMAD: Zdn = Za + Zdn * Zm
    // Uses ternary operation Fmad: (multiplicand, multiplier, addend)
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmad,
        predicate: Box::new(predicate),
        left: Box::new(multiplicand.clone()),  // Zdn - multiplicand
        middle: Box::new(multiplier),           // Zm - multiplier
        right: Box::new(addend),                // Za - addend
        destination: Box::new(multiplicand),    // Zdn - destination
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 1, // Zdn
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAD".to_string(),
            encoding: Some("Floating-point fused multiply-add to multiplicand (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmad_z_p_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmad_sve_predicated_validates() {
    let instruction = fmad_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FMAD SVE predicated should validate");
}

#[test]
fn fmad_sve_predicated_structure() {
    let instruction = fmad_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap3 for FMAD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fmad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 3, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}