/// VFMA (vector): Vector floating-point fused multiply-add
/// Stanford source: vfma_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vfma_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };
    // VFMA (vector): Vector floating-point fused multiply-add
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd,
        left: Box::new(mul),
        right: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VFMA (vector)".to_string(),
            encoding: Some("Vector floating-point fused multiply-add".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vfma_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vfma_vector_validates() {
    let instruction = vfma_vector_instruction();
    validate_instruction(&instruction).expect("VFMA (vector) should validate");
}

#[test]
fn vfma_vector_uses_fma() {
    let instruction = vfma_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for VFMA (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Check left is Mul
    let Expr::VectorMap2 { op: mul_op, left: mul_left, right: mul_right, .. } = left.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::FPMul);
    assert!(matches!(mul_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(mul_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}
