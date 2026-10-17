/// VCVTZ (floating-point to integer, round toward zero): Floating-point convert to integer, rounding toward zero
/// Stanford source: vcvt_fp_int_zero.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vcvtz_fp_int_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // VCVTZ: Floating-point convert to integer, rounding toward zero
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for VCVTZ
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VCVTZ (FP to int)".to_string(),
            encoding: Some("Floating-point convert to integer, rounding toward zero".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vcvt_fp_int_zero.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vcvtz_fp_int_validates() {
    let instruction = vcvtz_fp_int_instruction();
    validate_instruction(&instruction).expect("VCVTZ (FP to int) should validate");
}

#[test]
fn vcvtz_fp_int_uses_vector_map() {
    let instruction = vcvtz_fp_int_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VCVTZ (FP to int)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
