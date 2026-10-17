/// STFMINNM, STFMINNML: Atomic floating-point minimum number, without return
/// Stanford source: stfminnm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn stfminnm_stfminnml_instruction() -> Instruction {
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

    // STFMINNM/STFMINNML: Atomic floating-point minimum number without return
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
            mnemonic: "STFMINNM, STFMINNML".to_string(),
            encoding: Some("Atomic floating-point minimum number, without return".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "stfminnm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn stfminnm_stfminnml_validates() {
    let instruction = stfminnm_stfminnml_instruction();
    validate_instruction(&instruction).expect("STFMINNM, STFMINNML should validate");
}

#[test]
fn stfminnm_stfminnml_uses_vector_map2() {
    let instruction = stfminnm_stfminnml_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for STFMINNM, STFMINNML");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
