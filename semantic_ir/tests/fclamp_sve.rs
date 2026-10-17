/// FCLAMP (SVE unpredicated) - Floating-point clamp to minimum/maximum number
/// Stanford source: arm64_md_clean/fclamp_z_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   var result : bits(VL);
///   let operand1 : bits(VL) = Z[](n);  // minimum values
///   let operand2 : bits(VL) = Z[](m);  // maximum values
///   let operand3 : bits(VL) = Z[](d);  // input values to clamp
///   for e = 0 to elements-1 do
///       let element1 : bits(esize) = operand1[e*:esize];
///       let element2 : bits(esize) = operand2[e*:esize];
///       let element3 : bits(esize) = operand3[e*:esize];
///       let maxelement : bits(esize) = FPMaxNum(esize)(element1, element3, FPCR());
///       result[e*esize] = FPMinNum(esize)(maxelement, element2, FPCR());
///   end;
///   Z[](d) = result;
///
/// Clamps each element of Z[d] to be between corresponding elements of Z[n] (min) and Z[m] (max).
/// Result: clamp(Z[d], Z[n], Z[m]) = min(max(Z[d], Z[n]), Z[m])
/// This instruction is unpredicated.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fclamp_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - minimum
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Z[m] - maximum
    let operand3 = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[d] - input to clamp
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[d] (destructive)

    // FCLAMP: clamp(operand3, operand1, operand2) = min(max(operand3, operand1), operand2)
    // Using ternary operation FClamp
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::FClamp,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // No predicate - unpredicated
        left: Box::new(operand1),     // minimum (Z[n])
        middle: Box::new(operand2),   // maximum (Z[m])
        right: Box::new(operand3),    // input to clamp (Z[d])
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // Unpredicated
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Z[d]
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCLAMP".to_string(),
            encoding: Some("Floating-point clamp to minimum/maximum number (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fclamp_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fclamp_sve_unpredicated_validates() {
    let instruction = fclamp_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FCLAMP SVE unpredicated should validate");
}

#[test]
fn fclamp_sve_unpredicated_structure() {
    let instruction = fclamp_sve_unpredicated_instruction();

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
        panic!("expected ScalableVectorMap3 for FCLAMP");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::FClamp);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate is a zero constant (unpredicated)
    matches!(predicate.as_ref(), Expr::Const { value: 0, ty: BitWidth::W32 });
}