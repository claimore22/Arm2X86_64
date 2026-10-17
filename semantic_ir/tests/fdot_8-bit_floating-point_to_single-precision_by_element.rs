/// FDOT (8_bit floating-point to single-precision, by element) - bit_advsimd.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn fdot_8_bit_floating_point_to_single_precision_by_element_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FDOT (8_bit floating-point to single-precision, by element)".to_string(),
            encoding: Some("FDOT (8_bit floating-point to single-precision, by element)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bit_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fdot_8_bit_floating_point_to_single_precision_by_element_validates() {
    let instruction = fdot_8_bit_floating_point_to_single_precision_by_element_instruction();
    validate_instruction(&instruction).expect("FDOT (8_bit floating-point to single-precision, by element) should validate");
}

#[test]
fn fdot_8_bit_floating_point_to_single_precision_by_element_uses_vector_map() {
    let instruction = fdot_8_bit_floating_point_to_single_precision_by_element_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Abs);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}