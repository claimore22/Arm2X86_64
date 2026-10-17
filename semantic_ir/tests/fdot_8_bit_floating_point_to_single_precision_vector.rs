/// FDOT (8-bit floating-point to single-precision, vector): 8-bit floating-point dot product to single-precision (vector)
/// Stanford source: fdot_vec_single.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fdot_8_bit_floating_point_to_single_precision_vector_instruction() -> Instruction {
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
        reg: Reg::V(0),
        width: BitWidth::W128,
    };

    // FDOT (8-bit to single-precision, vector): Dot product with 8-bit FP elements
    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::FP8Dot,
        a: Box::new(left),
        b: Box::new(right),
        addend: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::SystemReg { reg: 0 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FDOT (8-bit floating-point to single-precision, vector)".to_string(),
            encoding: Some("8-bit floating-point dot product to single-precision (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fdot_vec_single.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fdot_8_bit_floating_point_to_single_precision_vector_validates() {
    let instruction = fdot_8_bit_floating_point_to_single_precision_vector_instruction();
    validate_instruction(&instruction).expect("FDOT (8-bit to single-precision, vector) should validate");
}

#[test]
fn fdot_8_bit_floating_point_to_single_precision_vector_uses_vector_dot() {
    let instruction = fdot_8_bit_floating_point_to_single_precision_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index } = value else {
        panic!("expected VectorDot for FDOT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::FP8Dot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(broadcast_index.is_none());
}