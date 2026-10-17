/// UMAX (register): Unsigned maximum (register)
/// Stanford source: umax_reg.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn umax_reg_instruction() -> Instruction {
    let datasize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let right = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // UMAX (register): Unsigned maximum register
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::UMax,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W64,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UMAX (register)".to_string(),
            encoding: Some("Unsigned maximum (register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "umax_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn umax_reg_validates() {
    let instruction = umax_reg_instruction();
    validate_instruction(&instruction).expect("UMAX (register) should validate");
}

#[test]
fn umax_reg_uses_vector_map2() {
    let instruction = umax_reg_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for UMAX (register)");
    };

    assert_eq!(*datasize, BitWidth::W64);
    assert_eq!(*esize, BitWidth::W64);
}