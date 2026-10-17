/// SM3SS1: SM3SS1
/// Stanford source: sm3ss1.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sm3ss1_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // SM3SS1: SM3 step 1
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for SM3SS1
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SM3SS1".to_string(),
            encoding: Some("SM3SS1".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SM3".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sm3ss1.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sm3ss1_validates() {
    let instruction = sm3ss1_instruction();
    validate_instruction(&instruction).expect("SM3SS1 should validate");
}

#[test]
fn sm3ss1_uses_vector_map2() {
    let instruction = sm3ss1_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SM3SS1");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}
