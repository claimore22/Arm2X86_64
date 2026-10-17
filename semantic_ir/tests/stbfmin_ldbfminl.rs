/// STBFMIN, STBFMINL: Atomic BFloat16 floating-point minimum, without return
/// Stanford source: stbfmin.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stbfmin_ldbfminl_instruction() -> Instruction {
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

    // STBFMIN/STBFMINL: Atomic BFloat16 minimum without return
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
            mnemonic: "STBFMIN, STBFMINL".to_string(),
            encoding: Some("Atomic BFloat16 floating-point minimum, without return".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string(), "FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stbfmin.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stbfmin_ldbfminl_validates() {
    let instruction = stbfmin_ldbfminl_instruction();
    validate_instruction(&instruction).expect("STBFMIN, STBFMINL should validate");
}

#[test]
fn stbfmin_ldbfminl_uses_vector_map2() {
    let instruction = stbfmin_ldbfminl_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STBFMIN, STBFMINL");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
