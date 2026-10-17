/// BFDOT (vector) - BFloat16 dot product to single-precision (vector)
/// Stanford source: arm64_md_clean/bfdot_advsimd_vec.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   let operand3 : bits(datasize) = V{}(d);
///   for e = 0 to elements-1 do
///       elt1_a = operand1[(2*e+0)*16];
///       elt1_b = operand1[(2*e+1)*16];
///       elt2_a = operand2[(2*e+0)*16];
///       elt2_b = operand2[(2*e+1)*16];
///       sum = operand3[e*32];
///       result[e*32] = BFDotAdd(sum, elt1_a, elt1_b, elt2_a, elt2_b, FPCR());
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bfdot_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let operand3 = Expr::ReadReg {
        reg: Reg::V(4),
        width: BitWidth::W128,
    };

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::BFDot,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(operand3),
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
            mnemonic: "BFDOT".to_string(),
            encoding: Some("BFloat16 dot product to single-precision (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfdot_advsimd_vec.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfdot_advsimd_validates() {
    let instruction = bfdot_advsimd_instruction();
    validate_instruction(&instruction).expect("BFDOT should validate");
}

#[test]
fn bfdot_advsimd_uses_vector_dot() {
    let instruction = bfdot_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    
    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index: _ } = value else {
        panic!("expected VectorDot");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::BFDot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(a.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(b.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
    assert!(matches!(addend.as_ref(), Expr::ReadReg { reg: Reg::V(4), width: BitWidth::W128 }));
    assert!(fpmr.is_none());
    assert!(matches!(fpcr.as_ref(), Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }));
}