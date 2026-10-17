/// FCADD (SVE predicated) - Floating-point complex add
/// Stanford source: arm64_md_clean/fcadd_z_p_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let pairs : integer = VL DIV (2 * esize);
///   let mask : bits(PL) = P[](g);
///   let operand1 : bits(VL) = Z[](dn);
///   let operand2 : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](m) else Zeros(VL);
///   var result : bits(VL);
///   for p = 0 to pairs-1 do
///       var acc_r : bits(esize) = operand1[(2*p + 0)*esize];
///       var acc_i : bits(esize) = operand1[(2*p + 1)*esize];
///       if ActivePredicateElement(PL)(mask, 2*p + 0, esize) then
///           var elt2_i : bits(esize) = operand2[(2*p + 1)*esize];
///           if sub_i then elt2_i = FPNeg(esize)(elt2_i, FPCR()); end;
///           acc_r = FPAdd(esize)(acc_r, elt2_i, FPCR());
///       end;
///       if ActivePredicateElement(PL)(mask, 2*p + 1, esize) then
///           var elt2_r : bits(esize) = operand2[(2*p + 0)*esize];
///           if sub_r then elt2_r = FPNeg(esize)(elt2_r, FPCR()); end;
///           acc_i = FPAdd(esize)(acc_i, elt2_r, FPCR());
///       end;
///       result[(2*p + 0)*esize] = acc_r;
///       result[(2*p + 1)*esize] = acc_i;
///   end;
///   Z[](dn) = result.
///
/// Complex numbers: even elements = real, odd elements = imaginary
/// Rotation 90° (rot='0'): sub_i=true, sub_r=false -> multiply by +j
/// Rotation 270° (rot='1'): sub_i=false, sub_r=true -> multiply by -j
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcadd_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (complex: real, imag, real, imag...)
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm (complex: real, imag, real, imag...)
    let destination = Expr::ReadScalableVecReg { reg: 1, vl }; // Zdn (destructive)

    // FCADD uses ternary operation Fcadd:
    // left = predicate, middle = Zdn, right = Zm
    // The operation handles complex cross-addition with rotation internally
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fcadd,
        predicate: Box::new(predicate),
        left: Box::new(operand1),     // Zdn - first source (complex pairs)
        middle: Box::new(operand2),   // Zm - second source (complex pairs)
        right: Box::new(destination.clone()), // Zdn - addend/destination
        destination: Box::new(destination), // Zdn - destination for merging
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
            mnemonic: "FCADD".to_string(),
            encoding: Some("Floating-point complex add (predicated, rot=90)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcadd_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcadd_sve_predicated_validates() {
    let instruction = fcadd_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FCADD SVE predicated should validate");
}

#[test]
fn fcadd_sve_predicated_structure() {
    let instruction = fcadd_sve_predicated_instruction();

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
        panic!("expected ScalableVectorMap3 for FCADD");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Fcadd);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}