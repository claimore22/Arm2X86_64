/// ADDV - Add across vector
/// Stanford source: arm64_md_clean/addv_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   [V]{esize}(d) = IntReduce{esize, datasize}(ReduceOp_ADD, operand);
///   // Adds all elements and produces a scalar result
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn addv_vector_instruction() -> Instruction {
    // Example: 128-bit vector with 64-bit elements (2 elements), reduces to 64-bit scalar
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // VectorReduce: add all elements together
    let result = Expr::VectorReduce {
        op: semantic_ir::primitives::VectorReduceOp::AddV,
        operand: Box::new(operand),
        datasize, esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),  // Scalar result written to X register
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDV".to_string(),
            encoding: Some("Add across vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addv_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addv_vector_validates() {
    let instruction = addv_vector_instruction();
    validate_instruction(&instruction).expect("ADDV vector should validate");
}

#[test]
fn addv_vector_uses_vector_reduce() {
    let instruction = addv_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorReduce { op, operand, datasize, esize } = value else {
        panic!("expected VectorReduce");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorReduceOp::AddV);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}

#[test]
fn addv_vector_writes_scalar_result() {
    let instruction = addv_vector_instruction();
    
    let Stmt::WriteReg { reg, value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Result written to X register (64-bit scalar)
    assert!(matches!(reg, Reg::X(1)));
    
    // Value is VectorReduce with result type 64-bit
    let Expr::VectorReduce { datasize, esize, .. } = value else {
        panic!("expected VectorReduce");
    };
    
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}