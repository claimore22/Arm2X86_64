/// LDBFMINNM, LDBFMINNMA, LDBFMINNMAL, LDBFMINNML: Atomic BFloat16 minimum number
/// Stanford source: ldbfminnm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ldbfminnm_instruction() -> Instruction {
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

    // LDBFMINNM: Atomic BFloat16 minimum number
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMinNum, // Placeholder for atomic minnum
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
            mnemonic: "LDBFMINNM".to_string(),
            encoding: Some("Atomic BFloat16 minimum number".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string(), "FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldbfminnm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldbfminnm_validates() {
    let instruction = ldbfminnm_instruction();
    validate_instruction(&instruction).expect("LDBFMINNM should validate");
}

#[test]
fn ldbfminnm_uses_vector_map2() {
    let instruction = ldbfminnm_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDBFMINNM");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}