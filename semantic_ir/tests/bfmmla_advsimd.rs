/// BFMMLA (widening) - BFloat16 matrix multiply-accumulate to single-precision
/// Stanford source: arm64_md_clean/bfmmla_advsimd.md
/// 
/// Stanford pseudocode:
///   let op1 : bits(128) = V{}(n);
///   let op2 : bits(128) = V{}(m);
///   let acc : bits(128) = V{}(d);
///   V{128}(d) = BFMatMulAddH(acc, op1, op2, FPCR());
///   // 2x4 matrix (op1) * 4x2 matrix (op2) = 2x2 matrix result (4x FP32)
///   // Accumulated into destination (4x FP32)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bfmmla_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // Result is 4x FP32

    let op1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let op2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let acc = Expr::ReadReg {
        reg: Reg::V(4),
        width: BitWidth::W128,
    };

    // BF16 Matrix multiply-accumulate
    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::BFDot,
        a: Box::new(op1),
        b: Box::new(op2),
        addend: Box::new(acc),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFMMLA".to_string(),
            encoding: Some("BFloat16 matrix multiply-accumulate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfmmla_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfmmla_advsimd_validates() {
    let instruction = bfmmla_advsimd_instruction();
    validate_instruction(&instruction).expect("BFMMLA should validate");
}

#[test]
fn bfmmla_advsimd_uses_vector_dot() {
    let instruction = bfmmla_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    
    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index: _ } = value else {
        panic!("expected VectorDot");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::BFDot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    matches!(a.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 });
    matches!(b.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 });
    matches!(addend.as_ref(), Expr::ReadReg { reg: Reg::V(4), width: BitWidth::W128 });
    assert!(matches!(fpmr, None));
    matches!(fpcr.as_ref(), Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 });
}