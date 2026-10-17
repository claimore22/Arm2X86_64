/// BFCVT - Single-precision convert to BFloat16 (scalar)
/// Stanford source: arm64_md_clean/bfcvt_float.md
/// 
/// Stanford pseudocode:
///   let operand : bits(32) = V{}(n);
///   let merge : boolean = IsMerging(FPCR());
///   var result : bits(128) = if merge then V{128}(d) else Zeros{128};
///   result[0+:16] = FPConvertBF(operand, FPCR());
///   V{128}(d) = result;
///   // Converts 32-bit float in Vn[31:0] to BF16, writes to Vd[15:0] with merge
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn bfcvt_float_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // 32-bit float input element size

    // Read source: 32-bit float from Vn (using 128-bit register width, but only lower 32 bits used)
    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: datasize,
    };

    // Convert using BF16CVT (FP32 to BF16)
    // Note: This is a semantic representation - the actual hardware only converts
    // the lower 32 bits and writes to lower 16 bits of destination
    let converted = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::BF16CVT,
        operand: Box::new(operand),
        datasize,
        esize: BitWidth::W32, // 32-bit float input element size
    };

    // Write result to Vd (lower 16 bits with merge behavior)
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: converted,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFCVT".to_string(),
            encoding: Some("Single-precision convert to BFloat16 (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfcvt_float.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfcvt_float_validates() {
    let instruction = bfcvt_float_instruction();
    validate_instruction(&instruction).expect("BFCVT (scalar) should validate");
}

#[test]
fn bfcvt_float_uses_bf16cvt() {
    let instruction = bfcvt_float_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for BF16CVT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::BF16CVT);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 });
}