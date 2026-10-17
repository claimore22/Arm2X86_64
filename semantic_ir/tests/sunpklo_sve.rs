/// SUNPKLO - Unpack and sign-extend low half
/// Stanford source: arm64_md_clean/sunpklo_z_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sunpklo_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Sunpklo,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SUNPKLO".to_string(),
            encoding: Some("Unpack and sign-extend low half".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sunpklo_z_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sunpklo_sve_validates() {
    let instruction = sunpklo_sve_instruction();
    validate_instruction(&instruction).expect("SUNPKLO SVE should validate");
}
