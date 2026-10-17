/// SHA256SU1: SHA-256 schedule update 1
/// Stanford source: sha256su1.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sha256su1_instruction() -> Instruction {
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

    // SHA256SU1: SHA-256 schedule update 1
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for SHA256SU1
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
            mnemonic: "SHA256SU1".to_string(),
            encoding: Some("SHA-256 schedule update 1".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SHA256".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sha256su1.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sha256su1_validates() {
    let instruction = sha256su1_instruction();
    validate_instruction(&instruction).expect("SHA256SU1 should validate");
}

#[test]
fn sha256su1_uses_vector_map2() {
    let instruction = sha256su1_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SHA256SU1");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}
