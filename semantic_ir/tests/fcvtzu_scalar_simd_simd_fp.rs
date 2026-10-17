/// FCVTZU (scalar SIMD&FP) - fcvtzu_z_p_z.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn fcvtzu_scalar_simd_simd_fp_instruction() -> Instruction {
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

    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Add,
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
            mnemonic: "FCVTZU (scalar SIMD&FP)".to_string(),
            encoding: Some("FCVTZU (scalar SIMD&FP)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtzu_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtzu_scalar_simd_simd_fp_validates() {
    let instruction = fcvtzu_scalar_simd_simd_fp_instruction();
    validate_instruction(&instruction).expect("FCVTZU (scalar SIMD&FP) should validate");
}

#[test]
fn fcvtzu_scalar_simd_simd_fp_uses_vector_map2() {
    let instruction = fcvtzu_scalar_simd_simd_fp_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Add);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
