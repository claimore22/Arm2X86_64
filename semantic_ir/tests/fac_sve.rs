/// FAC<cc> (SVE) - Floating-point absolute compare
/// Stanford source: arm64_md_clean/facge_p_p_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   let operand1 : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](n) else Zeros(VL);
///   let operand2 : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](m) else Zeros(VL);
///   var result : bits(PL);
///   let psize : integer = esize DIV 8;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let element1 : bits(esize) = FPAbs(esize)(operand1[e*:esize], FPCR());
///           let element2 : bits(esize) = FPAbs(esize)(operand2[e*:esize], FPCR());
///           var res : boolean;
///           case cmp_op of
///               when Cmp_GE => res = FPCompareGE(esize)(element1, element2, FPCR());
///               when Cmp_GT => res = FPCompareGT(esize)(element1, element2, FPCR());
///           end;
///           let pbit : bit = if res then '1' else '0';
///           result[e*psize] = ZeroExtend(psize)(pbit);
///       else
///           result[e*psize] = ZeroExtend(psize)('0');
///       end;
///   end;
///   P[](d) = result;
///
/// This compares absolute values of floating-point elements and produces predicate results.
/// FACGE: |Z[n]| >= |Z[m]|
/// FACGT: |Z[n]| > |Z[m]|
/// FACLT: |Z[n]| < |Z[m]| (pseudo-instruction: swap operands, use FACGT)
/// FACLE: |Z[n]| <= |Z[m]| (pseudo-instruction: swap operands, use FACGE)
/// FACEQ: |Z[n]| == |Z[m]| (pseudo-instruction: use FCMEQ on absolute values)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn facge_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    // FACGE compares absolute values: |Z[n]| >= |Z[m]|
    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Facge,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false, // Inactive elements set to zero
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FACGE".to_string(),
            encoding: Some("Floating-point absolute compare greater than or equal".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "facge_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn facgt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    // FACGT compares absolute values: |Z[n]| > |Z[m]|
    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Facgt,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FACGT".to_string(),
            encoding: Some("Floating-point absolute compare greater than".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "facge_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn facge_sve_validates() {
    let instruction = facge_sve_instruction();
    validate_instruction(&instruction).expect("FACGE SVE should validate");
}

#[test]
fn facge_sve_structure() {
    let instruction = facge_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap2 {
        op,
        left,
        right,
        pl,
        merging,
    } = value else {
        panic!("expected PredicateMap2 for FACGE");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Facge);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}

#[test]
fn facgt_sve_validates() {
    let instruction = facgt_sve_instruction();
    validate_instruction(&instruction).expect("FACGT SVE should validate");
}

#[test]
fn facgt_sve_structure() {
    let instruction = facgt_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap2 {
        op,
        left,
        right,
        pl,
        merging,
    } = value else {
        panic!("expected PredicateMap2 for FACGT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Facgt);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}