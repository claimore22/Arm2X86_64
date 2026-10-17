/// FAMAX: Floating-point absolute maximum
/// Stanford source: famax_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn famax_instruction() -> Instruction {
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

    // FAMAX: Floating-point absolute maximum
    // Uses FPMaxNum on absolute values
    let abs_left = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FPAbs,
        operand: Box::new(left),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let abs_right = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::FPAbs,
        operand: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMaxNum,
        left: Box::new(abs_left),
        right: Box::new(abs_right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FAMAX".to_string(),
            encoding: Some("Floating-point absolute maximum".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "famax_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn famax_validates() {
    let instruction = famax_instruction();
    validate_instruction(&instruction).expect("FAMAX should validate");
}

#[test]
fn famax_uses_vector_map2() {
    let instruction = famax_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FAMAX");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPMaxNum);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    // Check left is FPAbs
    let Expr::VectorMap { op: left_op, .. } = left.as_ref() else {
        panic!("expected VectorMap for left abs");
    };
    assert_eq!(*left_op, semantic_ir::primitives::VectorUnaryOp::FPAbs);
    // Check right is FPAbs
    let Expr::VectorMap { op: right_op, .. } = right.as_ref() else {
        panic!("expected VectorMap for right abs");
    };
    assert_eq!(*right_op, semantic_ir::primitives::VectorUnaryOp::FPAbs);
}