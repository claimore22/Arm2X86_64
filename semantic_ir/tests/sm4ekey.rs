/// SM4EKEY: SM4 key
/// Stanford source: sm4ekey.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sm4ekey_instruction() -> Instruction {
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

    // SM4EKEY: SM4 key
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for SM4EKEY
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
            mnemonic: "SM4EKEY".to_string(),
            encoding: Some("SM4 key".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SM4".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sm4ekey.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sm4ekey_validates() {
    let instruction = sm4ekey_instruction();
    validate_instruction(&instruction).expect("SM4EKEY should validate");
}

#[test]
fn sm4ekey_uses_vector_map2() {
    let instruction = sm4ekey_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SM4EKEY");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}
