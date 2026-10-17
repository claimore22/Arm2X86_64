/// VMINNM (vector): Vector floating-point minimum number
/// Stanford source: vminnm_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vminnm_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    // VMINNM (vector): Vector floating-point minimum number
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMinNum,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VMINNM (vector)".to_string(),
            encoding: Some("Vector floating-point minimum number".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vminnm_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vminnm_vector_validates() {
    let instruction = vminnm_vector_instruction();
    validate_instruction(&instruction).expect("VMINNM (vector) should validate");
}

#[test]
fn vminnm_vector_uses_fpminnum() {
    let instruction = vminnm_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for VMINNM (vector)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPMinNum);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}