/// FCMLT (zero): Floating-point compare less than zero (vector)
/// Stanford source: fcmlt_zero_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcmlt_zero_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCMLT (zero): Compare floating-point less than zero
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::CMLTZero,
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
            mnemonic: "FCMLT (zero)".to_string(),
            encoding: Some("Floating-point compare less than zero (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmlt_zero_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcmlt_zero_validates() {
    let instruction = fcmlt_zero_instruction();
    validate_instruction(&instruction).expect("FCMLT (zero) should validate");
}

#[test]
fn fcmlt_zero_uses_vector_map() {
    let instruction = fcmlt_zero_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCMLT (zero)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::CMLTZero);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}