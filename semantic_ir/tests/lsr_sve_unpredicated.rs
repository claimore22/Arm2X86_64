/// LSR (immediate, unpredicated) - Logical shift right by immediate (unpredicated)
/// Stanford source: arm64_md_clean/lsr_z_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorUnaryOp,
    validate_instruction,
};

fn lsr_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let shift = 2u16;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: SVEVectorUnaryOp::Lsr,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LSR".to_string(),
            encoding: Some("Logical shift right by immediate (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "lsr_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn lsr_sve_unpredicated_validates() {
    let instruction = lsr_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("LSR SVE unpredicated should validate");
}