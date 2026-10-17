/// UDOT (SVE): Dot product unsigned arithmetic (SVE)
/// Stanford source: udot_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn udot_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let a = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };
    let b = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 };
    let addend = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 };

    // UDOT (SVE): Unsigned dot product
    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::UDot,
        a: Box::new(a),
        b: Box::new(b),
        addend: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::SystemReg { reg: 0 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UDOT (SVE)".to_string(),
            encoding: Some("Dot product unsigned arithmetic (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "udot_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn udot_sve_validates() {
    let instruction = udot_sve_instruction();
    validate_instruction(&instruction).expect("UDOT (SVE) should validate");
}

#[test]
fn udot_sve_uses_vector_dot() {
    let instruction = udot_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index } = value else {
        panic!("expected VectorDot for UDOT (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::UDot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(broadcast_index.is_none());
}