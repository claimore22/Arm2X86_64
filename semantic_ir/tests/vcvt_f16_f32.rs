/// VCVT (FP16 to FP32): Half-precision to single-precision convert
/// Stanford source: vcvt_f16_f32.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vcvt_f16_f32_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    // VCVT: Half-precision to single-precision convert
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for VCVT
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
            mnemonic: "VCVT (F16 to F32)".to_string(),
            encoding: Some("Half-precision to single-precision convert".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vcvt_f16_f32.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vcvt_f16_f32_validates() {
    let instruction = vcvt_f16_f32_instruction();
    validate_instruction(&instruction).expect("VCVT (F16 to F32) should validate");
}

#[test]
fn vcvt_f16_f32_uses_vector_map() {
    let instruction = vcvt_f16_f32_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VCVT (F16 to F32)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
