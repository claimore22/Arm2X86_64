/// UDOT (by element): Dot product unsigned arithmetic (vector, by element)
/// Stanford source: udot_by_element.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn udot_by_element_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let a = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let b = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(0),
        width: BitWidth::W128,
    };

    // UDOT (by element): Unsigned dot product with broadcast element
    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::UDot,
        a: Box::new(a),
        b: Box::new(b),
        addend: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::SystemReg { reg: 0 }),
        fpmr: None,
        broadcast_index: Some(0),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UDOT (by element)".to_string(),
            encoding: Some("Dot product unsigned arithmetic (vector, by element)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "udot_by_element.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn udot_by_element_validates() {
    let instruction = udot_by_element_instruction();
    validate_instruction(&instruction).expect("UDOT (by element) should validate");
}

#[test]
fn udot_by_element_uses_vector_dot() {
    let instruction = udot_by_element_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index } = value else {
        panic!("expected VectorDot for UDOT (by element)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::UDot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(broadcast_index.is_some());
}
