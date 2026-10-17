/// FMAXQV - Floating-point maximum reduction of quadword vector segments
/// Stanford source: arm64_md_clean/fmaxqv_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fmaxqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::FMaxQV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAXQV".to_string(),
            encoding: Some("Floating-point maximum reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmaxqv_sve_validates() {
    let instruction = fmaxqv_sve_instruction();
    validate_instruction(&instruction).expect("FMAXQV SVE should validate");
}