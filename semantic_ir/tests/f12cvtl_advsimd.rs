/// F1CVTL, F1CVTL2, F2CVTL, F2CVTL2 - 8-bit floating-point convert to half-precision (vector)
/// Stanford source: arm64_md_clean/f12cvtl_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(64) = Vpart{}(n, part);
///   var result : bits(128);
///   for e = 0 to elements-1 do
///       result[e*16] = FP8ConvertFP(operand[e*8], issrc2, FPCR(), FPMR());
///   end;
///   V{128}(d) = result;
///   // Converts 8-bit FP elements to 16-bit FP with downscaling
///   // issrc2 selects F1 (false) vs F2 (true) format
///   // part selects lower (0) or upper (1) half of source
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn f1cvtl_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let src_esize = BitWidth::W8;
    let dst_esize = BitWidth::W16;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,  // V registers are 128-bit in the IR
    };

    // FP8 to FP16 conversion with downscaling
    // Note: This models the conversion of 8 elements from 8-bit FP8 to 16-bit FP16
    // The source is the lower 64 bits (8 elements of 8-bit FP8) of the 128-bit vector
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FP8CVT,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: src_esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "F1CVTL".to_string(),
            encoding: Some("8-bit FP convert to half-precision (lower half)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP8".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "f12cvtl_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn f1cvtl_advsimd_validates() {
    let instruction = f1cvtl_advsimd_instruction();
    validate_instruction(&instruction).expect("F1CVTL should validate");
}

#[test]
fn f1cvtl_advsimd_uses_fp8cvt() {
    let instruction = f1cvtl_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FP8CVT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FP8CVT);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W8);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}