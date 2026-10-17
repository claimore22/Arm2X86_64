/// ABS (predicated) - Absolute value (predicated)
/// Stanford source: arm64_md_clean/abs_z_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P(g);
///   let operand : bits(VL) = if AnyActiveElement(mask, esize) then Z(VL)(n) else Zeros{VL};
///   var result : bits(VL) = if merging then Z(VL)(d) else Zeros{VL};
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(mask, e, esize) then
///           var element : integer = SInt(operand[e*esize]);
///           element = Abs(element);
///           result[e*esize] = element[esize-1:0];
///       end;
///   end;
///   Z(VL)(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn abs_sve_predicated_instruction() -> Instruction {
    // Using VL=256, esize=32 (8 elements)
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32
    let esize = BitWidth::W32;
    let elements = vl.bits() / esize.bits(); // 8

    // Read predicate register P0
    let predicate = Expr::ReadPredReg {
        reg: 0,
        pl,
    };

    // Read source vector Z1
    let operand = Expr::ReadScalableVecReg {
        reg: 1,
        vl,
    };

    // Read destination vector Z0 (for merging predication)
    let destination = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };

    // ScalableVectorMap with SVEVectorUnaryOp::Abs
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Abs,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination), vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ABS".to_string(),
            encoding: Some("Absolute value (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "abs_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn abs_sve_predicated_validates() {
    let instruction = abs_sve_predicated_instruction();
    validate_instruction(&instruction).expect("ABS SVE predicated should validate");
}

#[test]
fn abs_sve_predicated_uses_scalable_vector_map() {
    let instruction = abs_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { op, predicate, operand, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap for ABS SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Abs);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
    assert!(matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
    assert!(matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
}

#[test]
fn abs_sve_predicated_element_count() {
    let instruction = abs_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap");
    };

    // VL=256, esize=32 -> 8 elements
    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 8);
}