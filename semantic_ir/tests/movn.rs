/// MOVN (Move Wide with NOT) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/movn.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, UnaryOp, ValueType, Width,
    validate_instruction,
};

fn movn_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // result = Zeros
    // pos = hw << 4 (0, 16, 32, or 48)
    // For example, hw = 2 -> pos = 32
    let pos = 32;

    // Place 16-bit immediate at position pos
    let imm16 = 0xABCD;
    let shifted_imm = Expr::Const {
        value: (imm16 as u128) << pos,
        ty: BitWidth::W64,
    };

    // MOVN does: result = NOT(result)
    let result = Expr::Unary {
        op: UnaryOp::Not,
        value: Box::new(shifted_imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVN".to_string(),
            encoding: Some("Move wide with NOT".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn movn_validates() {
    let instruction = movn_instruction();

    validate_instruction(&instruction).expect("MOVN should validate");
}

#[test]
fn movn_writes_result() {
    let instruction = movn_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn movn_uses_not_and_shifted_immediate() {
    let instruction = movn_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    let Expr::Unary { op, value: unary_value, .. } = value else {
        panic!("expected Unary Not");
    };

    assert_eq!(*op, UnaryOp::Not);

    // Inner value should be shifted immediate (constant)
    assert!(matches!(unary_value.as_ref(), Expr::Const { .. }));
}