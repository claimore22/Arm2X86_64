/// BF1CVTLT, BF2CVTLT - 8-bit floating-point convert to BFloat16 (top/odd elements)
/// Stanford source: arm64_md_clean/bf1cvtlt_z_z8.md
/// 
/// Stanford pseudocode:
///   let operand : bits(VL) = Z{}(n);
///   for e = 0 to elements-1 do
///       element = operand[(2*e + 1)*(esize/2)];
///       result[e*esize] = FP8ConvertBF(element, issrc2, FPCR(), FPMR());
///   end;
///   Z{VL}(d) = result;
///   // Only odd-numbered 8-bit elements are converted, placed in overlapping 16-bit elements
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bf1cvtlt_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W16;  // Result element size is 16-bit (BFloat16)
    let src_esize = BitWidth::W8;  // Source element size is 8-bit (FP8)

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // VectorDeinterleave: convert odd 8-bit elements to BF16
    let result = Expr::VectorDeinterleave {
        op: semantic_ir::primitives::VectorDeinterleaveOp::FP8ToBF16Deinterleave,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W8,
        scale: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 }),
        f8_format: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 }),
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 }),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BF1CVTLT".to_string(),
            encoding: Some("8-bit FP convert to BFloat16 (top/odd elements)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_FP8".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bf1cvtlt_z_z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bf1cvtlt_advsimd_validates() {
    let instruction = bf1cvtlt_advsimd_instruction();
    validate_instruction(&instruction).expect("BF1CVTLT should validate");
}

#[test]
fn bf1cvtlt_advsimd_uses_vector_deinterleave() {
    let instruction = bf1cvtlt_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    
    let Expr::VectorDeinterleave { op, operand, datasize, esize, .. } = value else {
        panic!("expected VectorDeinterleave");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorDeinterleaveOp::FP8ToBF16Deinterleave);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W8);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}