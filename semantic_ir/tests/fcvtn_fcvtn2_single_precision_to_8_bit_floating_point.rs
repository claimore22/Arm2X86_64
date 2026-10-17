/// FCVTN, FCVTN2 (single-precision to 8-bit floating-point): Single-precision convert to 8-bit floating-point (vector)
/// Stanford source: fcvtn_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcvtn_fcvtn2_single_precision_to_8_bit_floating_point_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTN/FCVTN2: Convert single-precision to 8-bit floating-point (narrowing)
    // Uses VectorMap with narrowing convert op
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for FCVTN
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
            mnemonic: "FCVTN, FCVTN2 (single-precision to 8-bit floating-point)".to_string(),
            encoding: Some("Single-precision convert to 8-bit floating-point (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtn_fcvtn2_single_precision_to_8_bit_floating_point_validates() {
    let instruction = fcvtn_fcvtn2_single_precision_to_8_bit_floating_point_instruction();
    validate_instruction(&instruction).expect("FCVTN, FCVTN2 (single-precision to 8-bit) should validate");
}

#[test]
fn fcvtn_fcvtn2_single_precision_to_8_bit_floating_point_uses_vector_map() {
    let instruction = fcvtn_fcvtn2_single_precision_to_8_bit_floating_point_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTN, FCVTN2");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}