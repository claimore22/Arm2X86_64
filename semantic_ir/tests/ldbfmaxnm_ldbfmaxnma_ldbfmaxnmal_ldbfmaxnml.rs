/// LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML: Atomic BFloat16 maximum number
/// Stanford source: ldbfmaxnm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ldbfmaxnm_instruction() -> Instruction {
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

    // LDBFMAXNM: Atomic BFloat16 maximum number
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMaxNum, // Placeholder for atomic maxnum
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
            mnemonic: "LDBFMAXNM".to_string(),
            encoding: Some("Atomic BFloat16 maximum number".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string(), "FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldbfmaxnm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldbfmaxnm_validates() {
    let instruction = ldbfmaxnm_instruction();
    validate_instruction(&instruction).expect("LDBFMAXNM should validate");
}

#[test]
fn ldbfmaxnm_uses_vector_map2() {
    let instruction = ldbfmaxnm_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDBFMAXNM");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}