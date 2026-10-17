/// FCM<cc> (SVE) - Floating-point compare (vectors)
/// Stanford source: arm64_md_clean/fcmeq_p_p_zz.md
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
///           let element1 : bits(esize) = operand1[e*:esize];
///           let element2 : bits(esize) = operand2[e*:esize];
///           var res : boolean;
///           case cmp_op of
///               when Cmp_EQ => res = FPCompareEQ(esize)(element1, element2, FPCR());
///               when Cmp_GE => res = FPCompareGE(esize)(element1, element2, FPCR());
///               when Cmp_GT => res = FPCompareGT(esize)(element1, element2, FPCR());
///               when Cmp_UN => res = FPCompareUN(esize)(element1, element2, FPCR());
///               when Cmp_NE => res = FPCompareNE(esize)(element1, element2, FPCR());
///               when Cmp_LT => res = FPCompareGT(esize)(element2, element1, FPCR());
///               when Cmp_LE => res = FPCompareGE(esize)(element2, element1, FPCR());
///           end;
///           let pbit : bit = if res then '1' else '0';
///           result[e*psize] = ZeroExtend(psize)(pbit);
///       else
///           result[e*psize] = ZeroExtend(psize)('0');
///       end;
///   end;
///   P[](d) = result;
///
/// This compares floating-point elements and produces predicate results.
/// FCMEQ: Z[n] == Z[m]
/// FCMGE: Z[n] >= Z[m]
/// FCMGT: Z[n] > Z[m]
/// FCMLE: Z[n] <= Z[m] (pseudo: swap operands, use FCMGE)
/// FCMLT: Z[n] < Z[m] (pseudo: swap operands, use FCMGT)
/// FCMNE: Z[n] != Z[m]
/// FCMUO: unordered(Z[n], Z[m])
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn fcmge_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Ge,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false, // Inactive elements set to zero
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMGE".to_string(),
            encoding: Some("Floating-point compare greater than or equal (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcmgt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Gt,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMGT".to_string(),
            encoding: Some("Floating-point compare greater than (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcmne_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Ne,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMNE".to_string(),
            encoding: Some("Floating-point compare not equal (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcmlt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    // FCMLT: Z[n] < Z[m] => swap operands and use Gt (Z[m] > Z[n])
    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Gt,
        left: Box::new(operand2), // swapped
        right: Box::new(operand1), // swapped
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMLT".to_string(),
            encoding: Some("Floating-point compare less than (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcmle_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    // FCMLE: Z[n] <= Z[m] => swap operands and use Ge (Z[m] >= Z[n])
    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Ge,
        left: Box::new(operand2), // swapped
        right: Box::new(operand1), // swapped
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMLE".to_string(),
            encoding: Some("Floating-point compare less than or equal (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcmuo_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Unordered,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCMUO".to_string(),
            encoding: Some("Floating-point compare unordered (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcmge_sve_validates() {
    let instruction = fcmge_sve_instruction();
    validate_instruction(&instruction).expect("FCMGE SVE should validate");
}

#[test]
fn fcmge_sve_structure() {
    let instruction = fcmge_sve_instruction();

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
        panic!("expected PredicateMap2 for FCMGE");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Ge);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}

#[test]
fn fcmgt_sve_validates() {
    let instruction = fcmgt_sve_instruction();
    validate_instruction(&instruction).expect("FCMGT SVE should validate");
}

#[test]
fn fcmgt_sve_structure() {
    let instruction = fcmgt_sve_instruction();

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
        panic!("expected PredicateMap2 for FCMGT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Gt);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}

#[test]
fn fcmne_sve_validates() {
    let instruction = fcmne_sve_instruction();
    validate_instruction(&instruction).expect("FCMNE SVE should validate");
}

#[test]
fn fcmne_sve_structure() {
    let instruction = fcmne_sve_instruction();

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
        panic!("expected PredicateMap2 for FCMNE");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Ne);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}

#[test]
fn fcmlt_sve_validates() {
    let instruction = fcmlt_sve_instruction();
    validate_instruction(&instruction).expect("FCMLT SVE should validate");
}

#[test]
fn fcmlt_sve_structure() {
    let instruction = fcmlt_sve_instruction();

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
        panic!("expected PredicateMap2 for FCMLT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Gt);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    // FCMLT swaps operands: Z[m] > Z[n]
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn fcmle_sve_validates() {
    let instruction = fcmle_sve_instruction();
    validate_instruction(&instruction).expect("FCMLE SVE should validate");
}

#[test]
fn fcmle_sve_structure() {
    let instruction = fcmle_sve_instruction();

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
        panic!("expected PredicateMap2 for FCMLE");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Ge);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    // FCMLE swaps operands: Z[m] >= Z[n]
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}

#[test]
fn fcmuo_sve_validates() {
    let instruction = fcmuo_sve_instruction();
    validate_instruction(&instruction).expect("FCMUO SVE should validate");
}

#[test]
fn fcmuo_sve_structure() {
    let instruction = fcmuo_sve_instruction();

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
        panic!("expected PredicateMap2 for FCMUO");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Unordered);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}