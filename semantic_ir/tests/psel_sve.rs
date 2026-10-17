/// PSEL - Predicate select between predicate register or all-false
/// Stanford source: arm64_md_clean/psel_p_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn psel_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadPredReg { reg: 1, pl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Psel,
        left: Box::new(predicate),
        right: Box::new(operand),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PSEL".to_string(),
            encoding: Some("Predicate select between predicate register or all-false".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "psel_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn psel_sve_validates() {
    let instruction = psel_sve_instruction();
    validate_instruction(&instruction).expect("PSEL SVE should validate");
}

#[test]
fn psel_sve_structure() {
    let instruction = psel_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap2 { op, left, right, pl, .. } = value else {
        panic!("expected PredicateMap2 for PSEL");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Psel);
    assert_eq!(*pl, BitWidth::W32);
    matches!(left.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(right.as_ref(), Expr::ReadPredReg { reg: 1, pl: BitWidth::W32 });
}