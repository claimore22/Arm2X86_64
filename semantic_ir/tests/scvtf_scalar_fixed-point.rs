/// SCVTF (scalar, fixed-point): Signed fixed-point convert to floating-point (scalar)
/// Stanford source: scvtf_scalar_fixed.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn scvtf_scalar_fixed_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // SCVTF (scalar, fixed-point): Convert signed fixed-point to floating-point
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for SCVTF
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
            mnemonic: "SCVTF (scalar, fixed-point)".to_string(),
            encoding: Some("Signed fixed-point convert to floating-point (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "scvtf_scalar_fixed.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn scvtf_scalar_fixed_validates() {
    let instruction = scvtf_scalar_fixed_instruction();
    validate_instruction(&instruction).expect("SCVTF (scalar, fixed-point) should validate");
}

#[test]
fn scvtf_scalar_fixed_uses_vector_map() {
    let instruction = scvtf_scalar_fixed_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for SCVTF (scalar, fixed-point)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}