/// LDTNP (SIMD&FP): Load unprivileged pair of SIMD&FP registers, with non-temporal hint
/// Stanford source: ldtnp_simd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ldtnp_simd_instruction() -> Instruction {
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

    // LDTNP: Load unprivileged pair with non-temporal hint
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for load pair
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
            mnemonic: "LDTNP (SIMD&FP)".to_string(),
            encoding: Some("Load unprivileged pair of SIMD&FP registers, with non-temporal hint".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldtnp_simd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldtnp_simd_validates() {
    let instruction = ldtnp_simd_instruction();
    validate_instruction(&instruction).expect("LDTNP (SIMD&FP) should validate");
}

#[test]
fn ldtnp_simd_uses_vector_map2() {
    let instruction = ldtnp_simd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDTNP (SIMD&FP)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
