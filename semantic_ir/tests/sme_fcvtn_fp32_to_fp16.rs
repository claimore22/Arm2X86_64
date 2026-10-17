/// FCVTN (FP32 to FP16): FCVTN (FP32 to FP16)
/// Stanford source: sme_fcvtn_fp32_to_fp16.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sme_fcvtn_fp32_to_fp16_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCVTN (FP32 to FP16): FCVTN (FP32 to FP16)
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTNS,
        operand: Box::new(operand),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCVTN (FP32 to FP16)".to_string(),
            encoding: Some("FCVTN (FP32 to FP16)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sme_fcvtn_fp32_to_fp16.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sme_fcvtn_fp32_to_fp16_validates() {
    let instruction = sme_fcvtn_fp32_to_fp16_instruction();
    validate_instruction(&instruction).expect("FCVTN (FP32 to FP16) should validate");
}

#[test]
fn sme_fcvtn_fp32_to_fp16_uses_vector_map() {
    let instruction = sme_fcvtn_fp32_to_fp16_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCVTN (FP32 to FP16)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTNS);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
