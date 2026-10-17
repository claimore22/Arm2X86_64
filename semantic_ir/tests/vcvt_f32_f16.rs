/// VCVT (FP32 to FP16): Single-precision to half-precision convert
/// Stanford source: vcvt_f32_f16.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vcvt_f32_f16_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W16;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    // VCVT: Single-precision to half-precision convert
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for VCVT
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W16,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VCVT (F32 to F16)".to_string(),
            encoding: Some("Single-precision to half-precision convert".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vcvt_f32_f16.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vcvt_f32_f16_validates() {
    let instruction = vcvt_f32_f16_instruction();
    validate_instruction(&instruction).expect("VCVT (F32 to F16) should validate");
}

#[test]
fn vcvt_f32_f16_uses_vector_map() {
    let instruction = vcvt_f32_f16_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VCVT (F32 to F16)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W16);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
