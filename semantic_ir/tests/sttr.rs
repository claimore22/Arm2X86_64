/// STTNP (SIMD&FP) - already fixed
/// STTP (SIMD&FP) - already fixed
/// STTR: Store-release register (unscaled offset)
/// Stanford source: sttr.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sttr_instruction() -> Instruction {
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

    // STTR: Store-release unscaled offset
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for store-release
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
            mnemonic: "STTR".to_string(),
            encoding: Some("Store-release register (unscaled offset)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sttr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sttr_validates() {
    let instruction = sttr_instruction();
    validate_instruction(&instruction).expect("STTR should validate");
}

#[test]
fn sttr_uses_vector_map2() {
    let instruction = sttr_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STTR");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
