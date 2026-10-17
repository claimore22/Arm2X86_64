/// FABD (SVE predicated) - Floating-point absolute difference
/// Stanford source: arm64_md_clean/fabd_z_p_zz.md
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
///           result[e*:esize] = FPAbs(esize)(FPSub(esize)(element1, element2, FPCR()), FPCR());
///       else
///           result[e*:esize] = element1;
///       end;
///   end;
///   Z[](dn) = result;
///
/// This is a destructive operation: Zdn is both source and destination.
/// For active elements: result = |Zdn - Zm|
/// For inactive elements: result = Zdn (unchanged)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fabd_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    // FABD supports esize = 16, 32, 64 (size != '00')
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fabd,
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
            mnemonic: "FABD".to_string(),
            encoding: Some("Floating-point absolute difference (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fabd_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fabd_sve_predicated_validates() {
    let instruction = fabd_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FABD SVE predicated should validate");
}

#[test]
fn fabd_sve_predicated_structure() {
    let instruction = fabd_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap2 for FABD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Fabd);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}