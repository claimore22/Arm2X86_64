/// FRECPE - Floating-point reciprocal estimate (AdvSIMD)
/// Stanford source: arm64_md_clean/frecpe_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn frecpe_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32; // single-precision
    let elements = 4; // 128 / 32 = 4

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FRecipEstimate,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
            esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRECPE".to_string(),
            encoding: Some("Floating-point reciprocal estimate (AdvSIMD)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frecpe_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frecpe_advsimd_validates() {
    let instruction = frecpe_advsimd_instruction();
    validate_instruction(&instruction).expect("FRECPE AdvSIMD should validate");
}