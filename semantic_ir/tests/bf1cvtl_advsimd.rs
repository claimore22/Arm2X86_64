/// BF1CVTL, BF2CVTL - Multi-vector 8-bit floating-point convert to deinterleaved BFloat16
/// Stanford source: arm64_md_clean/bf1cvtl_mz2_z8.md
/// 
/// Stanford pseudocode:
///   let operand : bits(VL) = Z{}(n);
///   for p = 0 to pairs-1 do
///       element1 = operand[(2*p + 0)*8];
///       element2 = operand[(2*p + 1)*8];
///       result1[p*16] = FP8ConvertBF(element1, issrc2, FPCR(), FPMR());
///       result2[p*16] = FP8ConvertBF(element2, issrc2, FPCR(), FPMR());
///   end;
///   Z{VL}(d+0) = result1; Z{VL}(d+1) = result2;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bf1cvtl_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;  // VL = 128
    let esize = BitWidth::W8;       // 8-bit FP8 elements

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // VectorDeinterleave: convert pairs of 8-bit FP8 to two BF16 vectors
    let result = Expr::VectorDeinterleave {
        op: semantic_ir::primitives::VectorDeinterleaveOp::FP8ToBF16Deinterleave,
        operand: Box::new(operand),
        datasize, esize,
        scale: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 }),
        f8_format: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 }),
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 }),
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::V(1),
            value: result.clone(),
        },
        Stmt::WriteReg {
            reg: Reg::V(3),  // d+1
            value: result,  // Same deinterleave produces both results
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "BF1CVTL".to_string(),
            encoding: Some("Multi-vector 8-bit FP convert to deinterleaved BFloat16".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SME2".to_string(), "FEAT_FP8".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bf1cvtl_mz2_z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bf1cvtl_advsimd_validates() {
    let instruction = bf1cvtl_advsimd_instruction();
    validate_instruction(&instruction).expect("BF1CVTL should validate");
}

#[test]
fn bf1cvtl_advsimd_uses_vector_deinterleave() {
    let instruction = bf1cvtl_advsimd_instruction();
    
    assert_eq!(instruction.semantics.len(), 2);
    
    for stmt in &instruction.semantics {
        let Stmt::WriteReg { value, .. } = stmt else {
            panic!("expected WriteReg");
        };
        
        let Expr::VectorDeinterleave { op, operand, datasize, esize, scale, f8_format, fpcr, fpmr, .. } = value else {
            panic!("expected VectorDeinterleave");
        };

        assert_eq!(*op, semantic_ir::primitives::VectorDeinterleaveOp::FP8ToBF16Deinterleave);
        assert_eq!(*datasize, BitWidth::W128);
        assert_eq!(*esize, BitWidth::W8);
        assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    }
}