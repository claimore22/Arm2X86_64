/// SM3TT1B: SM3TT1B
/// Stanford source: sm3tt1b.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sm3tt1b_instruction() -> Instruction {
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

    // SM3TT1B: SM3 T table 1B
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for SM3TT1B
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
            mnemonic: "SM3TT1B".to_string(),
            encoding: Some("SM3TT1B".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SM3".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sm3tt1b.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sm3tt1b_validates() {
    let instruction = sm3tt1b_instruction();
    validate_instruction(&instruction).expect("SM3TT1B should validate");
}

#[test]
fn sm3tt1b_uses_vector_map2() {
    let instruction = sm3tt1b_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SM3TT1B");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}