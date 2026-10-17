/// FDOT (SVE) - Floating-point dot product
/// Stanford sources:
///   - fdot_z_zz8z8.md (2-way, vectors, FP8 to FP16)
///   - fdot_z32_zz8z8.md (4-way, vectors, FP8 to FP32)
///   - fdot_z_zz8z8i.md (2-way, indexed, FP8 to FP16)
///   - fdot_z32_zz8z8i.md (4-way, indexed, FP8 to FP32)
///
/// FDOT (2-way, vectors, FP8 to FP16):
///   Each 16-bit element contains a pair of 8-bit FP values.
///   Dot product of each pair produces FP16 result, accumulated into Zda.
///   Zda = Zda + (Z[0]*Z[1] + Z[2]*Z[3] + ...) for each 16-bit element
///   Uses FP8DotAddFP with scaling by 2^-FPMR.LSCALE
///
/// FDOT (4-way, vectors, FP8 to FP32):
///   Each 32-bit element contains four 8-bit FP values.
///   Dot product produces FP32 result, accumulated into Zda.
///   Zda = Zda + (Z[0]*Z[1] + Z[2]*Z[3] + Z[4]*Z[5] + Z[6]*Z[7]) for each 32-bit element
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fdot_2way_fp16_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128; // SVE vector size (scalable, but dot product operates on fixed chunks)
    let esize = BitWidth::W16; // Result element size (16-bit FP16)

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 }; // Z[n] - FP8 pairs
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 }; // Z[m] - FP8 pairs
    let addend = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 };   // Z[da] - accumulator (FP16)

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::FP8Dot,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W16,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: Some(Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 })),
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Z[da]
        value: result,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FDOT".to_string(),
            encoding: Some("8-bit floating-point dot product to half-precision (2-way, vectors)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP8DOT2".to_string(), "FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fdot_z_zz8z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fdot_4way_fp32_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32; // Result element size (32-bit FP32)

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 }; // Z[n] - FP8 quads
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 }; // Z[m] - FP8 quads
    let addend = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 };   // Z[da] - accumulator (FP32)

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::FP8Dot,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: Some(Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpmr, width: BitWidth::W64 })),
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0, // Z[da]
        value: result,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FDOT".to_string(),
            encoding: Some("8-bit floating-point dot product to single-precision (4-way, vectors)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP8DOT4".to_string(), "FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fdot_z32_zz8z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fdot_2way_fp16_sve_validates() {
    let instruction = fdot_2way_fp16_sve_instruction();
    validate_instruction(&instruction).expect("FDOT 2-way FP16 SVE should validate");
}

#[test]
fn fdot_2way_fp16_sve_structure() {
    let instruction = fdot_2way_fp16_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot {
        op,
        a,
        b,
        addend,
        datasize,
        esize,
        fpcr,
        fpmr,
        broadcast_index,
    } = value else {
        panic!("expected VectorDot for FDOT 2-way FP16");
    };
    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::FP8Dot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W16);
    assert!(broadcast_index.is_none());
    assert!(fpmr.is_some());
    matches!(a.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 });
    matches!(b.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 });
    matches!(addend.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 });
}

#[test]
fn fdot_4way_fp32_sve_validates() {
    let instruction = fdot_4way_fp32_sve_instruction();
    validate_instruction(&instruction).expect("FDOT 4-way FP32 SVE should validate");
}

#[test]
fn fdot_4way_fp32_sve_structure() {
    let instruction = fdot_4way_fp32_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot {
        op,
        a,
        b,
        addend,
        datasize,
        esize,
        fpcr,
        fpmr,
        broadcast_index,
    } = value else {
        panic!("expected VectorDot for FDOT 4-way FP32");
    };
    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::FP8Dot);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(broadcast_index.is_none());
    assert!(fpmr.is_some());
    matches!(a.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 });
    matches!(b.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 });
    matches!(addend.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 });
}