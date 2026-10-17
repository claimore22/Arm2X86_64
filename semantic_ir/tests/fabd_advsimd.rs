/// FABD - Floating-point absolute difference (vector)
/// Stanford source: arm64_md_clean/fabd_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element1 = operand1[e*esize];
///       element2 = operand2[e*esize];
///       diff = FPSub(element1, element2, FPCR());
///       result[e*esize] = FPAbs(diff, FPCR());
///   end;
///   V{128}(d) = result;
///   // Computes |a - b| for each pair of FP elements
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fabd_advsimd_instruction() -> Instruction {
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

    // FABD: |a - b| = FPAbs(FPSub(a, b))
    // This is a vector binary operation that does FPSub then FPAbs
    let diff = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPSub,
        left: Box::new(operand1.clone()),
        right: Box::new(operand2.clone()),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FPAbs,
        operand: Box::new(diff),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FABD".to_string(),
            encoding: Some("Floating-point absolute difference (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fabd_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fabd_advsimd_validates() {
    let instruction = fabd_advsimd_instruction();
    validate_instruction(&instruction).expect("FABD should validate");
}

#[test]
fn fabd_advsimd_uses_fpsub_and_fpabs() {
    let instruction = fabd_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for FPAbs");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::FPAbs);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);

    let Expr::VectorMap2 { op: sub_op, left, right, .. } = operand.as_ref() else {
        panic!("expected VectorMap2 for FPSub");
    };

    assert_eq!(*sub_op, semantic_ir::primitives::VectorBinOp::FPSub);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}