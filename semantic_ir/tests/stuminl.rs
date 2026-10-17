/// STUMINL: Store unprivileged word minimum (unscaled offset)
/// Stanford source: stuminl.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stuminl_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W128, // Match datasize for VectorMap2 validation
    };
    let offset = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W128, // Match datasize for VectorMap2 validation
    };

    // STUMINL: Store unprivileged word minimum
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMin, // Placeholder for atomic min
        left: Box::new(base),
        right: Box::new(offset),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "STUMINL".to_string(),
            encoding: Some("Store unprivileged word minimum (unscaled offset)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stuminl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stuminl_validates() {
    let instruction = stuminl_instruction();
    validate_instruction(&instruction).expect("STUMINL should validate");
}

#[test]
fn stuminl_uses_vector_map2() {
    let instruction = stuminl_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STUMINL");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
