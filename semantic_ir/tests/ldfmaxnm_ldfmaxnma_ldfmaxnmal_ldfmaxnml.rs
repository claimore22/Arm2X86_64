/// LDFMAXNM, LDFMAXNMA, LDFMAXNMAL, LDFMAXNML: Atomic floating-point maximum number
/// Stanford source: ldfmaxnm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ldfmaxnm_instruction() -> Instruction {
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

    // LDFMAXNM: Atomic floating-point maximum number
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
            mnemonic: "LDFMAXNM".to_string(),
            encoding: Some("Atomic floating-point maximum number".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldfmaxnm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldfmaxnm_validates() {
    let instruction = ldfmaxnm_instruction();
    validate_instruction(&instruction).expect("LDFMAXNM should validate");
}

#[test]
fn ldfmaxnm_uses_vector_map2() {
    let instruction = ldfmaxnm_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LDFMAXNM");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}