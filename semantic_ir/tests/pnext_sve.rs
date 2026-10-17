/// PNEXT - Find next active predicate
/// Stanford source: arm64_md_clean/pnext_p_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn pnext_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadPredReg { reg: 1, pl };

    let result = Expr::PredicateMap {
        op: semantic_ir::primitives::SVEPredicateOp::Pnext,
        operand: Box::new(operand),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PNEXT".to_string(),
            encoding: Some("Find next active predicate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pnext_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pnext_sve_validates() {
    let instruction = pnext_sve_instruction();
    validate_instruction(&instruction).expect("PNEXT SVE should validate");
}

#[test]
fn pnext_sve_structure() {
    let instruction = pnext_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap { op, operand, pl, .. } = value else {
        panic!("expected PredicateMap for PNEXT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Pnext);
    assert_eq!(*pl, BitWidth::W32);
    matches!(operand.as_ref(), Expr::ReadPredReg { reg: 1, pl: BitWidth::W32 });
}