/// UADDLV: Unsigned sum long across vector
/// Stanford source: uaddlv.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uaddlv_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // UADDLV: Unsigned sum long across vector (reduction)
    let result = Expr::VectorReduce {
        op: semantic_ir::primitives::VectorReduceOp::UAddLV,
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
            mnemonic: "UADDLV".to_string(),
            encoding: Some("Unsigned sum long across vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uaddlv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uaddlv_validates() {
    let instruction = uaddlv_instruction();
    validate_instruction(&instruction).expect("UADDLV should validate");
}

#[test]
fn uaddlv_uses_vector_reduce() {
    let instruction = uaddlv_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorReduce { op, operand, datasize, esize } = value else {
        panic!("expected VectorReduce for UADDLV");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorReduceOp::UAddLV);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
