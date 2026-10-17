/// FCCMPE: Floating-point conditional signaling compare (scalar)
/// Stanford source: fccmpe_float.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, types::ValueType, types::Width, validate_instruction,
};

fn fccmpe_instruction() -> Instruction {
    let datasize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W64,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W64,
    };
    let nzcv = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W64,
    };

    // FCCMPE: Conditional signaling compare
    // Similar to FCCMP but signals invalid operation exception
    let cmp_result = Expr::ScalarFPBinary {
        op: semantic_ir::primitives::ScalarFPBinOp::FPSub,
        left: Box::new(left),
        right: Box::new(right),
        ty: BitWidth::W64,
    };

    let result = Expr::Select {
        condition: Box::new(Expr::Bool(true)), // Placeholder condition
        when_true: Box::new(cmp_result),
        when_false: Box::new(nzcv),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCCMPE".to_string(),
            encoding: Some("Floating-point conditional signaling compare (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fccmpe_float.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fccmpe_validates() {
    let instruction = fccmpe_instruction();
    validate_instruction(&instruction).expect("FCCMPE should validate");
}