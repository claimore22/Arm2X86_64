/// FMADD: Floating-point fused multiply-add (scalar)
/// Stanford source: fmadd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fmadd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // FMADD: Fused multiply-add
    // result = left * right + addend
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd,
        left: Box::new(mul),
        right: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMADD".to_string(),
            encoding: Some("Floating-point fused multiply-add (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmadd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmadd_validates() {
    let instruction = fmadd_instruction();
    validate_instruction(&instruction).expect("FMADD should validate");
}

#[test]
fn fmadd_uses_fused_mul_add() {
    let instruction = fmadd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Result is FPAdd( FPMul(left, right), addend )
    let Expr::VectorMap2 { op, left: mul, right: addend, datasize, esize } = value else {
        panic!("expected VectorMap2 for FMADD");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(addend.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Check mul is FPMul
    let Expr::VectorMap2 { op: mul_op, left, right, .. } = mul.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::FPMul);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}