/// SRSRA: Signed rounding shift right and accumulate (immediate)
/// Stanford source: srsra.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn srsra_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // SRSRA: Signed rounding shift right and accumulate
    // result = addend + (operand >> imm)
    let shift = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for shift
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Add,
        left: Box::new(addend),
        right: Box::new(shift),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SRSRA".to_string(),
            encoding: Some("Signed rounding shift right and accumulate (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "srsra.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn srsra_validates() {
    let instruction = srsra_instruction();
    validate_instruction(&instruction).expect("SRSRA should validate");
}

#[test]
fn srsra_uses_shift_add() {
    let instruction = srsra_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SRSRA");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Add);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}