/// MOVZ (Move Wide with Zero) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/movz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn movz_instruction() -> Instruction {
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

    // Since result starts as zeros, result = shifted_imm
    let result = shifted_imm;

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVZ".to_string(),
            encoding: Some("Move wide with zero".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn movz_validates() {
    let instruction = movz_instruction();

    validate_instruction(&instruction).expect("MOVZ should validate");
}

#[test]
fn movz_writes_result() {
    let instruction = movz_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn movz_uses_shifted_immediate() {
    let instruction = movz_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };

    // Value should be the shifted immediate constant
    assert!(matches!(value, Expr::Const { .. }));

    let Expr::Const { value, ty } = value else {
        panic!("expected Const");
    };

    // Should be 0xABCD << 32
    assert_eq!(*value, 0xABCD_u128 << 32);
    assert_eq!(*ty, BitWidth::W64);
}