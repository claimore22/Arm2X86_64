/// AND (predicates) - Bitwise AND predicates
/// Stanford source: arm64_md_clean/and_p_p_pp.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P(g);
///   let operand1 : bits(PL) = P(n);
///   let operand2 : bits(PL) = P(m);
///   var result : bits(PL);
///   let psize : integer = esize DIV 8;
///   for e = 0 to elements-1 do
///       let element1 : bit = PredicateElement(operand1, e, esize);
///       let element2 : bit = PredicateElement(operand2, e, esize);
///       if ActivePredicateElement(mask, e, esize) then
///           result[e*psize] = ZeroExtend(psize)(element1 AND element2);
///       else
///           result[e*psize] = ZeroExtend(psize)('0');
///       end;
///   end;
///   P(PL)(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn and_sve_predicates_instruction() -> Instruction {
    // Using VL=256, PL=32, esize=8 (predicate elements are 1-bit, 8 elements per byte)
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32
    let esize = BitWidth::W8;  // Predicate granularity is 8 bits

    // Read predicate register P0 (governing predicate)
    let predicate = Expr::ReadPredReg {
        reg: 0,
        pl,
    };

    // Read first source predicate P1
    let operand1 = Expr::ReadPredReg {
        reg: 1,
        pl,
    };

    // Read second source predicate P2
    let operand2 = Expr::ReadPredReg {
        reg: 2,
        pl,
    };

    // Read destination predicate P0 (for zeroing predication)
    let destination = Expr::ReadPredReg {
        reg: 0,
        pl,
    };

    // PredicateMap2 with SVEPredicateOp::And
    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::And,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false, // Zeroing predication
    };

    let semantics = vec![Stmt::WritePredReg {
        reg: 0,
        value: result,
        pl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AND".to_string(),
            encoding: Some("Bitwise AND predicates".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "and_p_p_pp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn and_sve_predicates_validates() {
    let instruction = and_sve_predicates_instruction();
    validate_instruction(&instruction).expect("AND SVE predicates should validate");
}

#[test]
fn and_sve_predicates_uses_predicate_map2() {
    let instruction = and_sve_predicates_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };

    let Expr::PredicateMap2 { op, left, right, pl, merging } = value else {
        panic!("expected PredicateMap2 for AND SVE predicates");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::And);
    assert_eq!(*pl, BitWidth::W32);
    assert_eq!(*merging, false);
    assert!(matches!(left.as_ref(), Expr::ReadPredReg { reg: 1, pl: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadPredReg { reg: 2, pl: BitWidth::W32 }));
}

#[test]
fn and_sve_predicates_element_count() {
    let instruction = and_sve_predicates_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };

    let Expr::PredicateMap2 { pl, .. } = value else {
        panic!("expected PredicateMap2");
    };

    // PL=32 bits, esize=8 -> 4 elements (32/8 = 4 predicate elements per byte? Actually PL is total bits)
    // For predicates, elements = PL / (esize/8) = 32 / 1 = 32 predicate bits
    // But esize here is the granularity, so elements = PL bits
    let _elements = pl.bits(); // 32 predicate bits
    assert_eq!(pl.bits(), 32);
}