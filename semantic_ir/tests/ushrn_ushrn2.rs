/// USHRN, USHRN2: Unsigned shift right narrow (immediate)
/// Stanford source: ushrn.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ushrn_ushrn2_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    // USHRN/USHRN2: Unsigned shift right narrow
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for USHRN
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
            mnemonic: "USHRN, USHRN2".to_string(),
            encoding: Some("Unsigned shift right narrow (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ushrn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ushrn_ushrn2_validates() {
    let instruction = ushrn_ushrn2_instruction();
    validate_instruction(&instruction).expect("USHRN, USHRN2 should validate");
}

#[test]
fn ushrn_ushrn2_uses_vector_map() {
    let instruction = ushrn_ushrn2_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for USHRN, USHRN2");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
