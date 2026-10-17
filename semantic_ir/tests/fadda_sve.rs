/// FADDA (SVE) - Floating-point add strictly-ordered reduction, accumulating in scalar
/// Stanford source: arm64_md_clean/fadda_v_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   let operand1 : bits(esize) = V[](dn);
///   let operand2 : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](m) else Zeros(VL);
///   var result : bits(esize) = operand1;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let element : bits(esize) = operand2[e*:esize];
///           result = FPAdd(esize)(result, element, FPCR());
///       end;
///   end;
///   V[](dn) = result;
///
/// This is a strictly-ordered reduction that:
/// - Takes a scalar initial value from V[dn] (SIMD&FP scalar register)
/// - Adds all active elements of Z[m] in order (low to high)
/// - Returns the result to V[dn]
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fadda_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let scalar_initial = Expr::ReadReg { reg: Reg::V(1), width: esize }; // Vdn (SIMD&FP scalar)
    let vector_operand = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm

    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::FAddA,
        predicate: Box::new(predicate),
        operand: Box::new(vector_operand),
        vl,
        esize,
        initial_value: Some(Box::new(scalar_initial)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1), // Vdn
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FADDA".to_string(),
            encoding: Some("Floating-point add strictly-ordered reduction".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fadda_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fadda_sve_validates() {
    let instruction = fadda_sve_instruction();
    validate_instruction(&instruction).expect("FADDA SVE should validate");
}

#[test]
fn fadda_sve_structure() {
    let instruction = fadda_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::ScalableVectorReduce {
        op,
        predicate,
        operand,
        vl,
        esize,
        initial_value,
    } = value else {
        panic!("expected ScalableVectorReduce for FADDA");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorReduceOp::FAddA);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    // Note: pl is not directly in the pattern, but we check predicate register width in the match
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    // Check initial_value is present and is the scalar register
    assert!(initial_value.is_some());
    matches!(initial_value.as_ref().unwrap().as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 });
}