/// FABS (scalar) - Floating-point absolute value (scalar)
/// Stanford source: arm64_md_clean/fabs_float.md
/// 
/// Stanford pseudocode:
///   let operand : bits(esize) = V{}(n);
///   var result : bits(128) = if IsMerging(FPCR()) then V{128}(d) else Zeros{128};
///   result[0+:esize] = FPAbs(operand, FPCR());
///   V{128}(d) = result;
///   // Computes absolute value of scalar FP value
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fabs_float_instruction() -> Instruction {
    let esize = BitWidth::W32;  // Single-precision
    let datasize = BitWidth::W128;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // FABS: absolute value of FP scalar
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FPAbs,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FABS".to_string(),
            encoding: Some("Floating-point absolute value (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fabs_float.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fabs_float_validates() {
    let instruction = fabs_float_instruction();
    validate_instruction(&instruction).expect("FABS (scalar) should validate");
}

#[test]
fn fabs_float_uses_fpabs() {
    let instruction = fabs_float_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FPAbs");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FPAbs);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}