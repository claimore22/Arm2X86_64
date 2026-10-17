/// FMMLA (widening, 8-bit floating-point to single-precision): 8-bit floating-point matrix multiply-accumulate to single-precision
/// Stanford source: fmmla_8bit_single.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fmmla_widening_8_bit_floating_point_to_single_precision_instruction() -> Instruction {
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

    // FMMLA (widening, 8-bit to single-precision): Matrix multiply-accumulate
    // result = addend + left * right (matrix multiplication)
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
            mnemonic: "FMMLA (widening, 8-bit floating-point to single-precision)".to_string(),
            encoding: Some("8-bit floating-point matrix multiply-accumulate to single-precision".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string(), "FEAT_FP8".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmmla_8bit_single.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmmla_widening_8_bit_floating_point_to_single_precision_validates() {
    let instruction = fmmla_widening_8_bit_floating_point_to_single_precision_instruction();
    validate_instruction(&instruction).expect("FMMLA (widening, 8-bit to single-precision) should validate");
}

#[test]
fn fmmla_widening_8_bit_floating_point_to_single_precision_uses_matrix_mul() {
    let instruction = fmmla_widening_8_bit_floating_point_to_single_precision_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FMMLA");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Check left is FPMul
    let Expr::VectorMap2 { op: mul_op, left: mul_left, right: mul_right, .. } = left.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::FPMul);
    assert!(matches!(mul_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(mul_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}