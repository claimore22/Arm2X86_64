/// BF1CVTL, BF2CVTL - Multi-vector 8-bit floating-point convert to deinterleaved BFloat16
/// Stanford source: arm64_md_clean/bf1cvtl_mz2_z8.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn bf1cvtl_bf2cvtl_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand = Expr::ReadScalableVecReg { reg: 2, vl }; // Zn - source

    // BF1CVTL/BF2CVTL: Multi-vector 8-bit FP convert to deinterleaved BFloat16
    // This is a VectorDeinterleave operation
    let result = Expr::VectorDeinterleave {
        op: semantic_ir::primitives::VectorDeinterleaveOp::FP8ToBF16Deinterleave,
        operand: Box::new(operand),
        datasize: vl,
        esize: BitWidth::W8,        // Source element size (8-bit FP8)
        scale: Box::new(Expr::ReadReg { reg: Reg::V(0), width: BitWidth::W64 }), // FPMR.LSCALE
        f8_format: Box::new(Expr::ReadReg { reg: Reg::V(0), width: BitWidth::W64 }), // FPMR.F8S1/F8S2
        fpcr: Box::new(Expr::ReadReg { reg: Reg::V(0), width: BitWidth::W64 }), // FPCR
        fpmr: Box::new(Expr::ReadReg { reg: Reg::V(0), width: BitWidth::W64 }), // FPMR
    };

    // Writes to two destination registers (Zd and Zd+1)
    let semantics = vec![
        Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl },  // Zd
        Stmt::WriteScalableVecReg { reg: 1, value: result, vl },          // Zd+1
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "BF1CVTL, BF2CVTL".to_string(),
            encoding: Some("Multi-vector 8-bit floating-point convert to deinterleaved BFloat16".to_string()),
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
fn bf1cvtl_bf2cvtl_validates() {
    let instruction = bf1cvtl_bf2cvtl_instruction();
    validate_instruction(&instruction).expect("BF1CVTL/BF2CVTL should validate");
}

#[test]
fn bf1cvtl_bf2cvtl_structure() {
    let instruction = bf1cvtl_bf2cvtl_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDeinterleave { op, operand, datasize, esize, scale, f8_format, fpcr, fpmr } = value else {
        panic!("expected VectorDeinterleave for BF1CVTL/BF2CVTL");
    };
    assert_eq!(*op, semantic_ir::primitives::VectorDeinterleaveOp::FP8ToBF16Deinterleave);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W8);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
}