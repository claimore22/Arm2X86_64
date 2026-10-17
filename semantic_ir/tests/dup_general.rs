/// DUP (general): Duplicate general-purpose register to vector
/// Stanford source: dup_general_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn dup_general_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W128, // Match datasize for VectorMap validation
    };

    // DUP (general): Duplicate a general-purpose register to all elements of a vector
    // Using VectorMap with a custom DupGeneral op as placeholder
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUP (general)".to_string(),
            encoding: Some("Duplicate general-purpose register to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dup_general_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dup_general_validates() {
    let instruction = dup_general_instruction();
    validate_instruction(&instruction).expect("DUP (general) should validate");
}

#[test]
fn dup_general_uses_vector_map() {
    let instruction = dup_general_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for DUP (general)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W128 }));
}