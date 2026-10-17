/// CMP<cc> (vectors) - Compare vectors (equal)
/// Stanford source: arm64_md_clean/cmpeq_p_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn cmp_sve_vectors_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;
    let vl = BitWidth::W256;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::PredicateMap2 {
        op: semantic_ir::primitives::SVEPredicateOp::Eq,
        left: Box::new(operand1),
        right: Box::new(operand2),
        pl,
        merging: false,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "CMP<cc> (vectors)".to_string(),
            encoding: Some("Compare vectors".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmpeq_p_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmp_sve_vectors_validates() {
    let instruction = cmp_sve_vectors_instruction();
    validate_instruction(&instruction).expect("CMP<cc> (vectors) SVE should validate");
}
