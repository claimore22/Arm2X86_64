/// AND (immediate) - Bitwise AND with immediate (unpredicated)
/// Stanford source: arm64_md_clean/and_z_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn and_sve_immediate_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Use a vector register to hold the immediate value (replicated for all elements)
    let immediate_vec = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::And,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(Expr::ReadScalableVecReg { reg: 1, vl }),
        right: Box::new(Expr::ReadScalableVecReg { reg: 2, vl }),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl }),
        vl,
        esize: BitWidth::W64,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "AND".to_string(),
            encoding: Some("Bitwise AND with immediate (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "and_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn and_sve_immediate_validates() {
    let instruction = and_sve_immediate_instruction();
    validate_instruction(&instruction).expect("AND SVE immediate should validate");
}