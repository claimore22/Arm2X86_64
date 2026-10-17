/// FMUL (immediate) - Floating-point multiply by immediate (predicated)
/// Stanford source: arm64_md_clean/fmul_z_p_zs.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn fmul_sve_immediate_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Use a vector register to hold the immediate value (+0.5 or +2.0)
    let immediate_vec = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Fmul,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl }),
        left: Box::new(operand1),
        right: Box::new(immediate_vec),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMUL".to_string(),
            encoding: Some("Floating-point multiply by immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmul_z_p_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmul_sve_immediate_validates() {
    let instruction = fmul_sve_immediate_instruction();
    validate_instruction(&instruction).expect("FMUL SVE immediate should validate");
}