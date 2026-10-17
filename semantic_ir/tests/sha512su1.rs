/// SHA512SU1: SHA-512 schedule update 1
/// Stanford source: sha512su1.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sha512su1_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // SHA512SU1: SHA-512 schedule update 1
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for SHA512SU1
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SHA512SU1".to_string(),
            encoding: Some("SHA-512 schedule update 1".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SHA512".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sha512su1.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sha512su1_validates() {
    let instruction = sha512su1_instruction();
    validate_instruction(&instruction).expect("SHA512SU1 should validate");
}

#[test]
fn sha512su1_uses_vector_map2() {
    let instruction = sha512su1_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SHA512SU1");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
