/// SABD: Signed absolute difference
/// Stanford source: sabd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sabd_instruction() -> Instruction {
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

    // SABD: Signed absolute difference
    let sub = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs,
        operand: Box::new(sub),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SABD".to_string(),
            encoding: Some("Signed absolute difference".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sabd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sabd_validates() {
    let instruction = sabd_instruction();
    validate_instruction(&instruction).expect("SABD should validate");
}

#[test]
fn sabd_uses_abs_sub() {
    let instruction = sabd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for SABD");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Abs);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);

    // Check operand is Sub
    let Expr::VectorMap2 { op: sub_op, left, right, .. } = operand.as_ref() else {
        panic!("expected VectorMap2 for Sub");
    };
    assert_eq!(*sub_op, semantic_ir::primitives::VectorBinOp::Sub);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}