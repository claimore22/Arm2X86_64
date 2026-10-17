/// PTRUES - Initialize predicate from named constraint and set condition flags
/// Stanford source: arm64_md_clean/ptrues_p_s.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ptrues_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let result = Expr::PredicateMap {
        op: semantic_ir::primitives::SVEPredicateOp::Ptrue,
        operand: Box::new(Expr::Const { value: 0x0F, ty: BitWidth::W32 }), // Pattern: ALL
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PTRUES".to_string(),
            encoding: Some("Initialize predicate from named constraint and set condition flags".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ptrues_p_s.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ptrues_sve_validates() {
    let instruction = ptrues_sve_instruction();
    validate_instruction(&instruction).expect("PTRUES SVE should validate");
}