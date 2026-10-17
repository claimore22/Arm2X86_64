/// FRECPX: Floating-point reciprocal exponent (scalar)
/// Stanford source: frecpx.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frecpx_instruction() -> Instruction {
    let datasize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FRECPX: Reciprocal exponent (extract exponent for reciprocal)
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for FRECPX
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
            mnemonic: "FRECPX".to_string(),
            encoding: Some("Floating-point reciprocal exponent (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frecpx.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frecpx_validates() {
    let instruction = frecpx_instruction();
    validate_instruction(&instruction).expect("FRECPX should validate");
}

#[test]
fn frecpx_uses_vector_map() {
    let instruction = frecpx_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FRECPX");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}