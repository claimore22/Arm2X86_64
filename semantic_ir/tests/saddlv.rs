/// SADDLV: Signed add long across vector
/// Stanford source: saddlv.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn saddlv_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // SADDLV: Signed add long across vector (reduction)
    let result = Expr::VectorReduce {
        op: semantic_ir::primitives::VectorReduceOp::AddV,
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
            mnemonic: "SADDLV".to_string(),
            encoding: Some("Signed add long across vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "saddlv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn saddlv_validates() {
    let instruction = saddlv_instruction();
    validate_instruction(&instruction).expect("SADDLV should validate");
}

#[test]
fn saddlv_uses_vector_reduce() {
    let instruction = saddlv_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorReduce { op, operand, datasize, esize } = value else {
        panic!("expected VectorReduce for SADDLV");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorReduceOp::AddV);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}