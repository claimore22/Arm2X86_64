/// EOR (predicates) - Bitwise exclusive-OR predicates
/// Stanford source: arm64_md_clean/eor_p_p_pp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn eor_sve_predicates_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadPredReg { reg: 1, pl };
    let operand2 = Expr::ReadPredReg { reg: 2, pl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Xor,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "EOR".to_string(),
            encoding: Some("Bitwise exclusive-OR predicates".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eor_p_p_pp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eor_sve_predicates_validates() {
    let instruction = eor_sve_predicates_instruction();
    validate_instruction(&instruction).expect("EOR SVE predicates should validate");
}

#[test]
fn eor_sve_predicates_uses_predicate_map2() {
    let instruction = eor_sve_predicates_instruction();
    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else { panic!("expected WritePredReg"); };
    let Expr::PredicateMap2 { op, left, right, pl, merging } = value else { panic!("expected PredicateMap2 for EOR predicates"); };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Xor);
    assert_eq!(*pl, BitWidth::W32);
    assert_eq!(*merging, false);
}