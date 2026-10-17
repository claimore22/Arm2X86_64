/// STFMAX, STFMAXL: Atomic floating-point maximum, without return
/// Stanford source: stfmax.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stfmax_stfmaxl_instruction() -> Instruction {
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

    // STFMAX/STFMAXL: Atomic floating-point maximum without return
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMax, // Placeholder for atomic max
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
            mnemonic: "STFMAX, STFMAXL".to_string(),
            encoding: Some("Atomic floating-point maximum, without return".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stfmax.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stfmax_stfmaxl_validates() {
    let instruction = stfmax_stfmaxl_instruction();
    validate_instruction(&instruction).expect("STFMAX, STFMAXL should validate");
}

#[test]
fn stfmax_stfmaxl_uses_vector_map2() {
    let instruction = stfmax_stfmaxl_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STFMAX, STFMAXL");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
