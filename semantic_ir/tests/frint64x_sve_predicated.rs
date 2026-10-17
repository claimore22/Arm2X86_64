/// FRINT64X - Floating-point round to 64-bit integer (predicated)
/// Stanford source: arm64_md_clean/frint64x_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorUnaryOp,
    validate_instruction,
};

fn frint64x_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Source element size (double-precision)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: SVEVectorUnaryOp::FRint64X,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination), vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRINT64X".to_string(),
            encoding: Some("Floating-point round to 64-bit integer (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frint64x_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frint64x_sve_predicated_validates() {
    let instruction = frint64x_sve_predicated_instruction();
    validate_instruction(&instruction).expect("FRINT64X SVE predicated should validate");
}