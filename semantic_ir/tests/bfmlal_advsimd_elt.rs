/// BFMLALB, BFMLALT (by element) - BFloat16 multiply-add to single-precision (vector, by element)
/// Stanford source: arm64_md_clean/bfmlal_advsimd_elt.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(128) = V{}(n);
///   let operand2 : bits(128) = V{}(m);
///   let operand3 : bits(128) = V{}(d);
///   let element2 : bits(16) = operand2[index*16];
///   for e = 0 to elements-1 do
///       element1 = operand1[(2*e + sel)*16];
///       addend = operand3[e*32];
///       result[e*32] = BFMulAddH(addend, element1, element2, FPCR());
///   end;
///   V{128}(d) = result;
///   // Uses broadcast of Vm[index] for multiply-add
///   // sel=0 for BFMLALB (even/bottom), sel=1 for BFMLALT (odd/top)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn bfmlalb_advsimd_elt_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;
    let imm_index = 0u8;  // Immediate index for element in Vm
    let sel = 0u8;        // 0 = BFMLALB (even/bottom), 1 = BFMLALT (odd/top)

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

    // VectorDot with BF16MulAdd operation and broadcast_index for by-element variant
    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::BF16MulAdd,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(operand3),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: None,
        broadcast_index: Some(imm_index),  // Immediate index for by-element variant
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFMLALB".to_string(),
            encoding: Some("BFloat16 multiply-add to single-precision (by element, bottom/even)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfmlal_advsimd_elt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn bfmlalt_advsimd_elt_instruction() -> Instruction {
    let mut instr = bfmlalb_advsimd_elt_instruction();
    instr.id.mnemonic = "BFMLALT".to_string();
    instr.id.encoding = Some("BFloat16 multiply-add to single-precision (by element, top/odd)".to_string());
    instr
}

#[test]
fn bfmlalb_advsimd_elt_validates() {
    let instruction = bfmlalb_advsimd_elt_instruction();
    validate_instruction(&instruction).expect("BFMLALB (by element) should validate");
}

#[test]
fn bfmlalt_advsimd_elt_validates() {
    let instruction = bfmlalt_advsimd_elt_instruction();
    validate_instruction(&instruction).expect("BFMLALT (by element) should validate");
}

#[test]
fn bfmlalb_advsimd_elt_uses_vector_dot_with_broadcast() {
    let instruction = bfmlalb_advsimd_elt_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    
    let Expr::VectorDot { op, a, b, addend, datasize, esize, fpcr, fpmr, broadcast_index } = value else {
        panic!("expected VectorDot");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::BF16MulAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(a.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(b.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
    assert!(matches!(addend.as_ref(), Expr::ReadReg { reg: Reg::V(4), width: BitWidth::W128 }));
    assert!(fpmr.is_none());
    assert!(matches!(fpcr.as_ref(), Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }));
    assert_eq!(*broadcast_index, Some(0u8));
}