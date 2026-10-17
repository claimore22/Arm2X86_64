/// FABS (vector) - Floating-point absolute value (vector)
/// Stanford source: arm64_md_clean/fabs_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element = operand[e*esize];
///       result[e*esize] = FPAbs(element, FPCR());
///   end;
///   V{datasize}(d) = result;
///   // Computes absolute value of each FP vector element
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fabs_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // Single-precision

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // FABS vector: absolute value of each element
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
            encoding: Some("Floating-point absolute value (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fabs_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fabs_advsimd_validates() {
    let instruction = fabs_advsimd_instruction();
    validate_instruction(&instruction).expect("FABS (vector) should validate");
}

#[test]
fn fabs_advsimd_uses_vector_map_fpabs() {
    let instruction = fabs_advsimd_instruction();
    
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