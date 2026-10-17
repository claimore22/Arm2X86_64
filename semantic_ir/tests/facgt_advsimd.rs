/// FACGT - Floating-point absolute compare greater than (vector)
/// Stanford source: arm64_md_clean/facgt_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element1 = operand1[e*esize];
///       element2 = operand2[e*esize];
///       element1 = FPAbs(element1, FPCR());
///       element2 = FPAbs(element2, FPCR());
///       test_passed = FPCompareGT(element1, element2, FPCR());
///       result[e*esize] = if test_passed then Ones{esize} else Zeros{esize};
///   end;
///   V{128}(d) = result;
///   // Compare |a| > |b| for each pair of FP elements
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn facgt_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // Single-precision

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // FACGT: |a| > |b| using FACGT which does FPAbs on both operands then compare
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FACGT,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FACGT".to_string(),
            encoding: Some("Floating-point absolute compare greater than (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "facgt_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn facgt_advsimd_validates() {
    let instruction = facgt_advsimd_instruction();
    validate_instruction(&instruction).expect("FACGT should validate");
}

#[test]
fn facgt_advsimd_uses_vector_map2_facgt() {
    let instruction = facgt_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FACGT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FACGT);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}