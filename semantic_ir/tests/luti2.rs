/// LUTI2: Lookup table read with 2-bit indices
/// Stanford source: luti2.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn luti2_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let table = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let indices = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // LUTI2: Lookup table with 2-bit indices
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for lookup
        left: Box::new(table),
        right: Box::new(indices),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LUTI2".to_string(),
            encoding: Some("Lookup table read with 2-bit indices".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "luti2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn luti2_validates() {
    let instruction = luti2_instruction();
    validate_instruction(&instruction).expect("LUTI2 should validate");
}

#[test]
fn luti2_uses_vector_map2() {
    let instruction = luti2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LUTI2");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
