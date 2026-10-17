/// UABDL, UABDL2: Unsigned absolute difference long
/// Stanford source: uabdl.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uabdl_uabdl2_instruction() -> Instruction {
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

    // UABDL/UABDL2: Unsigned absolute difference long
    let sub = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs,
        operand: Box::new(sub),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UABDL, UABDL2".to_string(),
            encoding: Some("Unsigned absolute difference long".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uabdl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uabdl_uabdl2_validates() {
    let instruction = uabdl_uabdl2_instruction();
    validate_instruction(&instruction).expect("UABDL, UABDL2 should validate");
}

#[test]
fn uabdl_uabdl2_uses_abs_sub() {
    let instruction = uabdl_uabdl2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for UABDL");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Abs);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);

    // Check operand is Sub
    let Expr::VectorMap2 { op: sub_op, left, right, .. } = operand.as_ref() else {
        panic!("expected VectorMap2 for Sub");
    };
    assert_eq!(*sub_op, semantic_ir::primitives::VectorBinOp::Sub);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}