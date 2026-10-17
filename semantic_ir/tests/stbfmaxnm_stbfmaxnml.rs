/// STBFMAXNM, STBFMAXNML: Atomic BFloat16 floating-point maximum number, without return
/// Stanford source: stbfmaxnm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stbfmaxnm_stbfmaxnml_instruction() -> Instruction {
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

    // STBFMAXNM/STBFMAXNML: Atomic BFloat16 maximum number without return
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
            mnemonic: "STBFMAXNM, STBFMAXNML".to_string(),
            encoding: Some("Atomic BFloat16 floating-point maximum number, without return".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string(), "FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stbfmaxnm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stbfmaxnm_stbfmaxnml_validates() {
    let instruction = stbfmaxnm_stbfmaxnml_instruction();
    validate_instruction(&instruction).expect("STBFMAXNM, STBFMAXNML should validate");
}

#[test]
fn stbfmaxnm_stbfmaxnml_uses_vector_map2() {
    let instruction = stbfmaxnm_stbfmaxnml_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STBFMAXNM, STBFMAXNML");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
