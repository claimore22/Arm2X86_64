/// FMAXV: Floating-point maximum across vector
/// Stanford source: fmaxv.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fmaxv_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // FMAXV: Maximum across vector (reduction)
    let result = Expr::VectorReduce {
        op: semantic_ir::primitives::VectorReduceOp::MaxNV,
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
            mnemonic: "FMAXV".to_string(),
            encoding: Some("Floating-point maximum across vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxv.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmaxv_validates() {
    let instruction = fmaxv_instruction();
    validate_instruction(&instruction).expect("FMAXV should validate");
}

#[test]
fn fmaxv_uses_vector_reduce() {
    let instruction = fmaxv_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorReduce { op, operand, datasize, esize } = value else {
        panic!("expected VectorReduce for FMAXV");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorReduceOp::MaxNV);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}