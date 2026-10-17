/// NAND - Bitwise NAND predicates
/// Stanford source: arm64_md_clean/nand_p_p_pp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn nand_sve_predicated_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadPredReg { reg: 1, pl };
    let operand2 = Expr::ReadPredReg { reg: 2, pl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Nand,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "NAND".to_string(),
            encoding: Some("Bitwise NAND predicates".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "nand_p_p_pp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn nand_sve_predicated_validates() {
    let instruction = nand_sve_predicated_instruction();
    validate_instruction(&instruction).expect("NAND SVE predicated should validate");
}

#[test]
fn nand_sve_predicated_uses_predicate_map2() {
    let instruction = nand_sve_predicated_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };

    let Expr::PredicateMap2 { op, left, right, pl, merging } = value else {
        panic!("expected PredicateMap2 for NAND");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Nand);
    assert_eq!(*pl, BitWidth::W32);
    assert!(!*merging);
    assert!(matches!(left.as_ref(), Expr::ReadPredReg { reg: 1, pl: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadPredReg { reg: 2, pl: BitWidth::W32 }));
}
