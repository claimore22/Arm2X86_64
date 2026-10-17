/// LDR (literal, SIMD&FP): Load SIMD&FP register (PC-relative literal)
/// Stanford source: ldr_literal_simd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ldr_literal_simd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    // LDR (literal): Load from PC-relative literal
    let result = Expr::Const {
        value: 0,
        ty: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDR (literal, SIMD&FP)".to_string(),
            encoding: Some("Load SIMD&FP register (PC-relative literal)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldr_literal_simd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldr_literal_simd_validates() {
    let instruction = ldr_literal_simd_instruction();
    validate_instruction(&instruction).expect("LDR (literal, SIMD&FP) should validate");
}

#[test]
fn ldr_literal_simd_uses_const() {
    let instruction = ldr_literal_simd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Const { ty, .. } = value else {
        panic!("expected Const for LDR (literal, SIMD&FP)");
    };

    assert_eq!(*ty, BitWidth::W128);
}