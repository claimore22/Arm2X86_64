/// CNTP (predicate as counter) - Set scalar to count from predicate-as-counter
/// Stanford source: arm64_md_clean/cntp_r_pn.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let pred : bits(PL) = P[n];
///   let mask : bits(PL*4) = CounterToPredicate(pred[15:0]);
///   var sum : bits(64) = Zeros{};
///   let maxelements : integer = elements * width;
///   for e = 0 to maxelements-1 do
///       if ActivePredicateElement{mask, e, esize} then
///           sum = sum + 1;
///       end;
///   end;
///   X[64](d) = sum;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn cntp_predicate_as_counter_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let n = 0u8; // Pn
    let d = 0u8; // Rd

    // Source predicate (16 bits used for counter)
    let pred = Expr::ReadPredReg { reg: n, pl };

    // CounterToPredicate: expands 16-bit predicate to PL*4 bits
    let mask = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CounterToPredicate,
        predicate: None,
        operands: vec![Box::new(pred)],
        vl: BitWidth::W256,
        esize: Some(BitWidth::W16), // 16-bit counter input
        imm: Some(4), // width = 4
    };

    let elements = (BitWidth::W256.bits() / 32) as u64; // esize = 32
    let maxelements = elements * 4; // width = 4

    // Count active elements in expanded mask up to maxelements
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CountPred,
        predicate: Some(Box::new(mask.clone())),
        operands: vec![Box::new(mask)],
        vl: BitWidth::W256,
        esize: Some(BitWidth::W32),
        imm: Some((elements * 4) as u16),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: semantic_ir::types::Reg::X(0),
        value: count,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CNTP".to_string(),
            encoding: Some("Set scalar to count from predicate-as-counter".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cntp_r_pn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cntp_predicate_as_counter_validates() {
    let instruction = cntp_predicate_as_counter_instruction();
    validate_instruction(&instruction).expect("CNTP (predicate as counter) SVE should validate");
}

#[test]
fn cntp_predicate_as_uses_counter_to_predicate() {
    let instruction = cntp_predicate_as_counter_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::SVESpecial { op, operands, .. } = value else {
        panic!("expected SVESpecial for CountPred");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::CountPred);

    // First operand should be the CounterToPredicate result
    let Expr::SVESpecial { op: ctp_op, .. } = operands[0].as_ref() else {
        panic!("expected CounterToPredicate SVESpecial");
    };

    assert_eq!(*ctp_op, semantic_ir::primitives::SVESpecialOp::CounterToPredicate);
}

#[test]
fn cntp_predicate_as_uses_counter_to_predicate_op() {
    let instruction = cntp_predicate_as_counter_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::SVESpecial { op, operands, .. } = value else {
        panic!("expected SVESpecial for CountPred");
    };

    let Expr::SVESpecial { op: ctp_op, operands: ctp_operands, .. } = operands[0].as_ref() else {
        panic!("expected CounterToPredicate SVESpecial");
    };

    assert_eq!(*ctp_op, semantic_ir::primitives::SVESpecialOp::CounterToPredicate);
    assert_eq!(ctp_operands.len(), 1);
    // CounterToPredicate takes the lower 16 bits of predicate
    assert!(matches!(ctp_operands[0].as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
}