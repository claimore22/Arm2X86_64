/// FCVTNU (scalar SIMD&FP): Floating-point convert to unsigned integer, rounding to nearest with ties to even (scalar SIMD&FP)
/// Stanford source: fcvt_nu_scalar_simd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtnu_scalar_simd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTNU (scalar SIMD&FP): Convert to unsigned integer, rounding to nearest with ties to even
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTNU,
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
            mnemonic: "FCVTNU (scalar SIMD&FP)".to_string(),
            encoding: Some("Floating-point convert to unsigned integer, rounding to nearest with ties to even (scalar SIMD&FP)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvt_nu_scalar_simd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtnu_scalar_simd_validates() {
    let instruction = fcvtnu_scalar_simd_instruction();
    validate_instruction(&instruction).expect("FCVTNU (scalar SIMD&FP) should validate");
}

#[test]
fn fcvtnu_scalar_simd_uses_vector_map() {
    let instruction = fcvtnu_scalar_simd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTNU (scalar SIMD&FP)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTNU);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}