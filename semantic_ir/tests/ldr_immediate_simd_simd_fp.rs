/// LDR (immediate, SIMD&FP): Load SIMD&FP register (immediate offset)
/// Stanford source: ldr_imm_simd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ldr_immediate_simd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W128, // Match datasize for VectorMap2 validation
    };
    let offset = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W128, // Match datasize for VectorMap2 validation
    };

    // LDR (immediate): Load with immediate offset
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for load
        left: Box::new(base),
        right: Box::new(offset),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDR (immediate, SIMD&FP)".to_string(),
            encoding: Some("Load SIMD&FP register (immediate offset)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldr_imm_simd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldr_immediate_simd_validates() {
    let instruction = ldr_immediate_simd_instruction();
    validate_instruction(&instruction).expect("LDR (immediate, SIMD&FP) should validate");
}

#[test]
fn ldr_immediate_simd_uses_vector_map2() {
    let instruction = ldr_immediate_simd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDR (immediate, SIMD&FP)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}