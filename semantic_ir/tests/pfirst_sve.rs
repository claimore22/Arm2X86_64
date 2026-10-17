/// PFIRST - Set the First active predicate element to true
/// Stanford source: arm64_md_clean/pfirst_p_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn pfirst_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    let result = Expr::PredicateMap {
        op: semantic_ir::primitives::SVEPredicateOp::Pfirst,
        operand: Box::new(predicate),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PFIRST".to_string(),
            encoding: Some("Set the First active predicate element to true".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pfirst_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pfirst_sve_validates() {
    let instruction = pfirst_sve_instruction();
    validate_instruction(&instruction).expect("PFIRST SVE should validate");
}

#[test]
fn pfirst_sve_structure() {
    let instruction = pfirst_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::PredicateMap { op, operand, pl, .. } = value else {
        panic!("expected PredicateMap for PFIRST");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEPredicateOp::Pfirst);
    assert_eq!(*pl, BitWidth::W32);
    matches!(operand.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}