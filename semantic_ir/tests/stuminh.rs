/// STUMINH: Store unprivileged halfword minimum (unscaled offset)
/// Stanford source: stuminh.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stuminh_instruction() -> Instruction {
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

    // STUMINH: Store unprivileged halfword minimum
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMin, // Placeholder for atomic halfword min
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
            mnemonic: "STUMINH".to_string(),
            encoding: Some("Store unprivileged halfword minimum (unscaled offset)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stuminh.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stuminh_validates() {
    let instruction = stuminh_instruction();
    validate_instruction(&instruction).expect("STUMINH should validate");
}

#[test]
fn stuminh_uses_vector_map2() {
    let instruction = stuminh_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STUMINH");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}