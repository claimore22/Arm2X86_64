/// PTRUE (predicate as counter) - Initialize predicate-as-counter to all active
/// Stanford source: arm64_md_clean/ptrue_pn_i.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ptrue_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let result = Expr::PredicateMap {
        op: semantic_ir::primitives::SVEPredicateOp::Ptrue,
        operand: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PTRUE".to_string(),
            encoding: Some("Initialize predicate-as-counter to all active".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ptrue_pn_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ptrue_sve_validates() {
    let instruction = ptrue_sve_instruction();
    validate_instruction(&instruction).expect("PTRUE SVE should validate");
}