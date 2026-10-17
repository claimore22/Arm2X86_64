/// BFCVT (scalar) - Single-precision convert to BFloat16 (scalar)
/// Stanford source: arm64_md_clean/bfcvt_float.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn bf_cvt_scalar_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128, // Match datasize for VectorMap validation
    };

    // BFCVT: Single-precision convert to BFloat16 (scalar)
    // Uses VectorMap with Fcvt unary op
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::BF16CVT,
        operand: Box::new(operand),
        datasize,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFCVT (scalar)".to_string(),
            encoding: Some("Single-precision convert to BFloat16 (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfcvt_float.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bf_cvt_scalar_validates() {
    let instruction = bf_cvt_scalar_instruction();
    validate_instruction(&instruction).expect("BFCVT (scalar) should validate");
}

#[test]
fn bf_cvt_scalar_structure() {
    let instruction = bf_cvt_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for BFCVT scalar");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::BF16CVT);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 });
}