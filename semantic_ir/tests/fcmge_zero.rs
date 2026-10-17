/// FCMGE (zero): Floating-point compare greater than or equal to zero (vector)
/// Stanford source: fcmge_zero_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcmge_zero_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FCMGE (zero): Compare floating-point greater than or equal to zero
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::CMGEZero,
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
            mnemonic: "FCMGE (zero)".to_string(),
            encoding: Some("Floating-point compare greater than or equal to zero (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcmge_zero_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcmge_zero_validates() {
    let instruction = fcmge_zero_instruction();
    validate_instruction(&instruction).expect("FCMGE (zero) should validate");
}

#[test]
fn fcmge_zero_uses_vector_map() {
    let instruction = fcmge_zero_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FCMGE (zero)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::CMGEZero);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}