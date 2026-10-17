/// FDOT (2-way, multiple and single vector, FP16 to FP32): FDOT (2-way, multiple and single vector, FP16 to FP32)
/// Stanford source: sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32_instruction() -> Instruction {
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

    // FDOT (2-way, multiple and single vector, FP16 to FP32): FDOT (2-way, multiple and single vector, FP16 to FP32)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul,
        left: Box::new(left),
        right: Box::new(right),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FDOT (2-way, multiple and single vector, FP16 to FP32)".to_string(),
            encoding: Some("FDOT (2-way, multiple and single vector, FP16 to FP32)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32_validates() {
    let instruction = sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32_instruction();
    validate_instruction(&instruction).expect("FDOT (2-way, multiple and single vector, FP16 to FP32) should validate");
}

#[test]
fn sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32_uses_vector_map2() {
    let instruction = sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FDOT (2-way, multiple and single vector, FP16 to FP32)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Mul);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}
