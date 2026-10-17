/// SUBHN, SUBHN2: Subtract returning high narrow
/// Stanford source: subhn.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn subhn_subhn2_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // SUBHN/SUBHN2: Subtract returning high narrow
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub, // Placeholder for SUBHN
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SUBHN, SUBHN2".to_string(),
            encoding: Some("Subtract returning high narrow".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "subhn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn subhn_subhn2_validates() {
    let instruction = subhn_subhn2_instruction();
    validate_instruction(&instruction).expect("SUBHN, SUBHN2 should validate");
}

#[test]
fn subhn_subhn2_uses_vector_map2() {
    let instruction = subhn_subhn2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SUBHN, SUBHN2");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}