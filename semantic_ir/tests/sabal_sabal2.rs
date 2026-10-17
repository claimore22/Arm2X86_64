/// SABAL, SABAL2: Signed absolute difference and accumulate long
/// Stanford source: sabal.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sabal_sabal2_instruction() -> Instruction {
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

    // SABAL/SABAL2: Signed absolute difference and accumulate long
    let sub = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let abs_sub = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs,
        operand: Box::new(sub),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Add,
        left: Box::new(addend),
        right: Box::new(abs_sub),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABAL, SABAL2".to_string(),
            encoding: Some("Signed absolute difference and accumulate long".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabal.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sabal_sabal2_validates() {
    let instruction = sabal_sabal2_instruction();
    validate_instruction(&instruction).expect("SABAL, SABAL2 should validate");
}

#[test]
fn sabal_sabal2_uses_abs_sub_add() {
    let instruction = sabal_sabal2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SABAL");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Add);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Check right is Abs(Sub)
    let Expr::VectorMap { op: abs_op, operand: sub, .. } = right.as_ref() else {
        panic!("expected VectorMap for Abs");
    };
    assert_eq!(*abs_op, semantic_ir::primitives::VectorUnaryOp::Abs);

    let Expr::VectorMap2 { op: sub_op, left: sub_left, right: sub_right, .. } = sub.as_ref() else {
        panic!("expected VectorMap2 for Sub");
    };
    assert_eq!(*sub_op, semantic_ir::primitives::VectorBinOp::Sub);
    assert!(matches!(sub_left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(sub_right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}