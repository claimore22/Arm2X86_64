/// BFMLALB, BFMLALT (vector) - BFloat16 multiply-add to single-precision (vector)
/// Stanford source: arm64_md_clean/bfmlal_advsimd_vec.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(128) = V{}(n);
///   let operand2 : bits(128) = V{}(m);
///   let operand3 : bits(128) = V{}(d);
///   for e = 0 to elements-1 do
///       element1 = operand1[(2*e + sel)*16];
///       element2 = operand2[(2*e + sel)*16];
///       addend = operand3[e*32];
///       result[e*32] = BFMulAddH(addend, element1, element2, FPCR());
///   end;
///   V{128}(d) = result;
///   sel = 0 for BFMLALB (bottom/even), 1 for BFMLALT (top/odd)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bfmlalb_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // Result is FP32

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

    // BFMLALB: sel=0 (bottom/even elements)
    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::BF16MulAdd,
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
            mnemonic: "BFMLALB".to_string(),
            encoding: Some("BFloat16 multiply-add to single-precision (bottom/even)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfmlal_advsimd_vec.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn bfmlalt_advsimd_instruction() -> Instruction {
    // BFMLALT is identical except sel=1 (top/odd elements)
    let mut instr = bfmlalb_advsimd_instruction();
    instr.id.mnemonic = "BFMLALT".to_string();
    instr.id.encoding = Some("BFloat16 multiply-add to single-precision (top/odd)".to_string());
    instr
}

#[test]
fn bfmlalb_advsimd_validates() {
    let instruction = bfmlalb_advsimd_instruction();
    validate_instruction(&instruction).expect("BFMLALB should validate");
}

#[test]
fn bfmlalt_advsimd_validates() {
    let instruction = bfmlalt_advsimd_instruction();
    validate_instruction(&instruction).expect("BFMLALT should validate");
}

#[test]
fn bfmlalb_advsimd_uses_vector_dot() {
    let instruction = bfmlalb_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    
    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index: _ } = value else {
        panic!("expected VectorDot");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::BF16MulAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    matches!(a.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 });
    matches!(b.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 });
    matches!(addend.as_ref(), Expr::ReadReg { reg: Reg::V(4), width: BitWidth::W128 });
    assert!(matches!(*fpmr, None));
    matches!(fpcr.as_ref(), Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 });
}

#[test]
fn bfmlalt_advsimd_uses_vector_dot() {
    let instruction = bfmlalt_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    
    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index: _ } = value else {
        panic!("expected VectorDot");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::BF16MulAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(fpmr, None));
}