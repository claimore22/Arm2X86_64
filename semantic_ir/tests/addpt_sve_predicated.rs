/// ADDPT (predicated) - Add checked pointer vectors (predicated)
/// Stanford source: arm64_md_clean/addpt_z_p_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV 64;
///   let mask : bits(PL) = P(g);
///   let operand1 : bits(VL) = Z(dn);
///   let operand2 : bits(VL) = if AnyActiveElement(mask, 64) then Z(m) else Zeros{VL};
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(64) = operand1[e*:64];
///       let element2 : bits(64) = operand2[e*:64];
///       if ActivePredicateElement(mask, e, 64) then
///           let res : bits(64) = element1 + element2;
///           result[e*:64] = PointerAddCheck(res, element1);
///       else
///           result[e*:64] = operand1[e*:64];
///       end;
///   end;
///   Z(VL)(dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn addpt_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Build: for each active element, result = PointerAddCheck(element1 + element2, element1)
    // Inactive elements: result = operand1 (merging predication)
    // The IR models this as ScalableVectorMap2 with Add, then PointerAddCheck on active elements.
    // Since PointerAddCheck is scalar-only, we model the predicated pointer-check add
    // as a ScalableVectorMap2 with a composite op that includes the check.
    // The existing IR has no SVE-specific pointer-add-check op, so we use Add
    // and note that PointerAddCheck is applied in the lowering.
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDPT".to_string(),
            encoding: Some("Add checked pointer vectors (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE".to_string(),
            "FEAT_CPA".to_string(),
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addpt_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addpt_sve_predicated_validates() {
    let instruction = addpt_sve_predicated_instruction();
    validate_instruction(&instruction).expect("ADDPT SVE predicated should validate");
}

#[test]
fn addpt_sve_predicated_uses_merging_predication() {
    let instruction = addpt_sve_predicated_instruction();

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
    } = value
    else {
        panic!("expected ScalableVectorMap2 for ADDPT SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Add);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(
        *predication,
        semantic_ir::primitives::PredicationMode::Merging
    );
    assert!(matches!(
        predicate.as_ref(),
        Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }
    ));
    assert!(matches!(
        left.as_ref(),
        Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        right.as_ref(),
        Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        destination.as_ref(),
        Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }
    ));
}

#[test]
fn addpt_sve_predicated_element_count() {
    let instruction = addpt_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 4);
}
