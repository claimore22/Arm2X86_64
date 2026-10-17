/// PFALSE - Set all predicate elements to false
/// Stanford source: arm64_md_clean/pfalse_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn pfalse_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let result = Expr::PredicateMap {
        op: semantic_ir::primitives::SVEPredicateOp::Pfalse,
        operand: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PFALSE".to_string(),
            encoding: Some("Set all predicate elements to false".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pfalse_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pfalse_sve_validates() {
    let instruction = pfalse_sve_instruction();
    validate_instruction(&instruction).expect("PFALSE SVE should validate");
}