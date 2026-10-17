/// ST1D (scalar plus scalar, tile slice): ST1D (scalar plus scalar, tile slice)
/// Stanford source: sme_st1d_scalar_plus_scalar_tile_slice.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sme_st1d_scalar_plus_scalar_tile_slice_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // ST1D (scalar plus scalar, tile slice): ST1D (scalar plus scalar, tile slice)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Add,
        left: Box::new(left),
        right: Box::new(right),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ST1D (scalar plus scalar, tile slice)".to_string(),
            encoding: Some("ST1D (scalar plus scalar, tile slice)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sme_st1d_scalar_plus_scalar_tile_slice.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sme_st1d_scalar_plus_scalar_tile_slice_validates() {
    let instruction = sme_st1d_scalar_plus_scalar_tile_slice_instruction();
    validate_instruction(&instruction).expect("ST1D (scalar plus scalar, tile slice) should validate");
}

#[test]
fn sme_st1d_scalar_plus_scalar_tile_slice_uses_vector_map2() {
    let instruction = sme_st1d_scalar_plus_scalar_tile_slice_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for ST1D (scalar plus scalar, tile slice)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Add);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}
