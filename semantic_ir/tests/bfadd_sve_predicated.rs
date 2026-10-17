/// BFADD (predicated) - BFloat16 add (predicated)
/// Stanford source: arm64_md_clean/bfadd_z_p_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV 16;
///   let mask : bits(PL) = P(g);
///   let operand1 : bits(VL) = Z(dn);
///   let operand2 : bits(VL) = if AnyActiveElement(mask, 16) then Z(VL)(m) else Zeros{VL};
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(16) = operand1[e*16];
///       if ActivePredicateElement(mask, e, 16) then
///           let element2 : bits(16) = operand2[e*16];
///           result[e*16] = BFAdd(16)(element1, element2, FPCR());
///       else
///           result[e*16] = element1;
///       end;
///   end;
///   Z(VL)(dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn bfadd_sve_predicated_instruction() -> Instruction {
    // Using VL=256, esize=16 (16 elements for BFloat16)
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32
    let esize = BitWidth::W16;  // BFloat16 is 16-bit

    // Read predicate register P0
    let predicate = Expr::ReadPredReg {
        reg: 0,
        pl,
    };

    // Read first source/destination vector Z0 (dn)
    let operand1 = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };

    // Read second source vector Z1 (m)
    let operand2 = Expr::ReadScalableVecReg {
        reg: 1,
        vl,
    };

    // Read destination vector Z0 (for merging predication)
    let destination = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };

    // ScalableVectorMap2 with SVEVectorBinOp::Bfloat16Add
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Bfloat16Add,
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
            mnemonic: "BFADD".to_string(),
            encoding: Some("BFloat16 add (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_B16B16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfadd_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfadd_sve_predicated_validates() {
    let instruction = bfadd_sve_predicated_instruction();
    validate_instruction(&instruction).expect("BFADD SVE predicated should validate");
}

#[test]
fn bfadd_sve_predicated_uses_scalable_vector_map2() {
    let instruction = bfadd_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for BFADD SVE predicated");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Bfloat16Add);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W16);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    assert!(matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
    assert!(matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
    assert!(matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
}

#[test]
fn bfadd_sve_predicated_element_count() {
    let instruction = bfadd_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    // VL=256, esize=16 -> 16 elements
    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 16);
}