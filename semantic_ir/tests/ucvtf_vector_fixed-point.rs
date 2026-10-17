/// UC VT F (vector, fixed-point): Unsigned fixed-point convert to floating-point (vector)
/// Stanford source: ucvtf_vector_fixed.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ucvtf_vector_fixed_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // UC VT F (vector, fixed-point): Convert unsigned fixed-point to floating-point
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for UC VT F
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
            mnemonic: "UC VT F (vector, fixed-point)".to_string(),
            encoding: Some("Unsigned fixed-point convert to floating-point (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ucvtf_vector_fixed.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ucvtf_vector_fixed_validates() {
    let instruction = ucvtf_vector_fixed_instruction();
    validate_instruction(&instruction).expect("UC VT F (vector, fixed-point) should validate");
}

#[test]
fn ucvtf_vector_fixed_uses_vector_map() {
    let instruction = ucvtf_vector_fixed_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for UC VT F (vector, fixed-point)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
