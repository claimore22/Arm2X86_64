/// AND (vector) - Bitwise AND (vector)
/// Stanford source: arm64_md_clean/and_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   V{datasize}(d) = operand1 AND operand2;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn and_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::And,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AND".to_string(),
            encoding: Some("Bitwise AND (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "and_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn and_vector_validates() {
    let instruction = and_vector_instruction();
    validate_instruction(&instruction).expect("AND vector should validate");
}

#[test]
fn and_vector_uses_vector_map2() {
    let instruction = and_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::And);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}