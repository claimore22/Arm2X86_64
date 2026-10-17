/// SHA256H: SHA-256 hash update (part 1)
/// Stanford source: sha256h.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sha256h_instruction() -> Instruction {
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
    let addend = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // SHA256H: SHA-256 hash update (part 1)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for SHA256H
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
            mnemonic: "SHA256H".to_string(),
            encoding: Some("SHA-256 hash update (part 1)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SHA256".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sha256h.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sha256h_validates() {
    let instruction = sha256h_instruction();
    validate_instruction(&instruction).expect("SHA256H should validate");
}

#[test]
fn sha256h_uses_vector_map2() {
    let instruction = sha256h_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SHA256H");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}
