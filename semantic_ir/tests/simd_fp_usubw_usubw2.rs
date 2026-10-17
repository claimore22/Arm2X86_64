/// USUBW, USUBW2: USUBW, USUBW2
/// Stanford source: simd_fp_usubw_usubw2.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn simd_fp_usubw_usubw2_instruction() -> Instruction {
    let datasize = BitWidth::W32;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W32,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W32,
    };

    // USUBW, USUBW2: USUBW, USUBW2
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::UQSub,
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
            mnemonic: "USUBW, USUBW2".to_string(),
            encoding: Some("USUBW, USUBW2".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "simd_fp_usubw_usubw2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn simd_fp_usubw_usubw2_validates() {
    let instruction = simd_fp_usubw_usubw2_instruction();
    validate_instruction(&instruction).expect("USUBW, USUBW2 should validate");
}

#[test]
fn simd_fp_usubw_usubw2_uses_vector_map2() {
    let instruction = simd_fp_usubw_usubw2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for USUBW, USUBW2");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::UQSub);
    assert_eq!(*datasize, BitWidth::W32);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W32 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W32 }));
}
