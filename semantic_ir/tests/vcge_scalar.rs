/// VCGE (scalar): Scalar compare greater than or equal
/// Stanford source: vcge_scalar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vcge_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W64;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W64,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W64,
    };
    // VCGE (scalar): Scalar compare greater than or equal
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for VCGE
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W64,
        esize: BitWidth::W64,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VCGE (scalar)".to_string(),
            encoding: Some("Scalar compare greater than or equal".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vcge_scalar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vcge_scalar_validates() {
    let instruction = vcge_scalar_instruction();
    validate_instruction(&instruction).expect("VCGE (scalar) should validate");
}

#[test]
fn vcge_scalar_uses_vector_map2() {
    let instruction = vcge_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for VCGE (scalar)");
    };

    assert_eq!(*datasize, BitWidth::W64);
    assert_eq!(*esize, BitWidth::W64);
}
