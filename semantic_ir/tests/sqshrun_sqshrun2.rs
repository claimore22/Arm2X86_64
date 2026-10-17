/// SQSHRUN, SQSHRUN2: Signed saturating shift right unsigned narrow (immediate)
/// Stanford source: sqshrun.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sqshrun_sqshrun2_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // SQSHRUN/SQSHRUN2: Signed saturating shift right unsigned narrow
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for SQSHRUN
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
            mnemonic: "SQSHRUN, SQSHRUN2".to_string(),
            encoding: Some("Signed saturating shift right unsigned narrow (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqshrun.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqshrun_sqshrun2_validates() {
    let instruction = sqshrun_sqshrun2_instruction();
    validate_instruction(&instruction).expect("SQSHRUN, SQSHRUN2 should validate");
}

#[test]
fn sqshrun_sqshrun2_uses_vector_map() {
    let instruction = sqshrun_sqshrun2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for SQSHRUN, SQSHRUN2");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}