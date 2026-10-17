/// BFCVT (BFloat16 to 8-bit floating-point): BFCVT (BFloat16 to 8-bit floating-point)
/// Stanford source: sme_bfcvt_bfloat16_to_8_bit_floating_point.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sme_bfcvt_bfloat16_to_8_bit_floating_point_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // BFCVT (BFloat16 to 8-bit floating-point): BFCVT (BFloat16 to 8-bit floating-point)
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FCVTZS,
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
            mnemonic: "BFCVT (BFloat16 to 8-bit floating-point)".to_string(),
            encoding: Some("BFCVT (BFloat16 to 8-bit floating-point)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sme_bfcvt_bfloat16_to_8_bit_floating_point.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sme_bfcvt_bfloat16_to_8_bit_floating_point_validates() {
    let instruction = sme_bfcvt_bfloat16_to_8_bit_floating_point_instruction();
    validate_instruction(&instruction).expect("BFCVT (BFloat16 to 8-bit floating-point) should validate");
}

#[test]
fn sme_bfcvt_bfloat16_to_8_bit_floating_point_uses_vector_map() {
    let instruction = sme_bfcvt_bfloat16_to_8_bit_floating_point_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for BFCVT (BFloat16 to 8-bit floating-point)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FCVTZS);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
