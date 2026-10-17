/// UMIN (register): Unsigned minimum (register)
/// Stanford source: umin_reg.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn umin_reg_instruction() -> Instruction {
    let datasize = BitWidth::W64;

    let left = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let right = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // UMIN (register): Unsigned minimum register
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::UMin,
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
            mnemonic: "UMIN (register)".to_string(),
            encoding: Some("Unsigned minimum (register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "umin_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn umin_reg_validates() {
    let instruction = umin_reg_instruction();
    validate_instruction(&instruction).expect("UMIN (register) should validate");
}

#[test]
fn umin_reg_uses_vector_map2() {
    let instruction = umin_reg_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for UMIN (register)");
    };

    assert_eq!(*datasize, BitWidth::W64);
    assert_eq!(*esize, BitWidth::W64);
}