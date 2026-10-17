/// UC VT F (scalar, fixed-point): Unsigned fixed-point convert to floating-point (scalar)
/// Stanford source: ucvtf_scalar_fixed.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ucvtf_scalar_fixed_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // UC VT F (scalar, fixed-point): Convert unsigned fixed-point to floating-point
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for UC VT F
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
            mnemonic: "UC VT F (scalar, fixed-point)".to_string(),
            encoding: Some("Unsigned fixed-point convert to floating-point (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ucvtf_scalar_fixed.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ucvtf_scalar_fixed_validates() {
    let instruction = ucvtf_scalar_fixed_instruction();
    validate_instruction(&instruction).expect("UC VT F (scalar, fixed-point) should validate");
}

#[test]
fn ucvtf_scalar_fixed_uses_vector_map() {
    let instruction = ucvtf_scalar_fixed_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for UC VT F (scalar, fixed-point)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}