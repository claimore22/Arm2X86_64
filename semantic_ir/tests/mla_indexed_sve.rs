/// MLA (indexed) - Multiply-add by indexed element (unpredicated)
/// Stanford source: arm64_md_clean/mla_z_zzzi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn mla_indexed_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    // Read first source vector Z0
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };

    // Read second source vector Z1
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };

    // Read addend/destination vector Z0
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Immediate index (0-1 for 64-bit in 128-bit segments)
    let imm_index = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Fmla,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // No predicate - unpredicated
        left: Box::new(imm_index),        // Immediate index
        middle: Box::new(operand1),       // First source Z(n)
        right: Box::new(operand2),        // Second source Z(m)
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MLA".to_string(),
            encoding: Some("Multiply-add by indexed element".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mla_z_zzzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mla_indexed_sve_validates() {
    let instruction = mla_indexed_sve_instruction();
    validate_instruction(&instruction).expect("MLA (indexed) SVE should validate");
}