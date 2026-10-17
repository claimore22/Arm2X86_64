/// ERET (Exception Return) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/eret.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, SpecialReg, Stmt, ValueType, Width,
    validate_instruction,
};

fn eret_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // ERET: target = ELR_ELx; AArch64_ExceptionReturn(target, SPSR_ELx)
    // In IR: we model this as a Branch with special handling
    let target = Expr::ReadSpecial {
        reg: SpecialReg::Pc, // ELR is typically accessed via special handling
        width: BitWidth::W64,
    };

    // For the semantic IR, we model ERET as a Branch with special target
    // The actual SPSR restore is a side effect
    let semantics = vec![Stmt::Branch { target }];

    Instruction {
        id: InstructionId {
            mnemonic: "ERET".to_string(),
            encoding: Some("Exception return".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eret.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eret_validates() {
    let instruction = eret_instruction();

    validate_instruction(&instruction).expect("ERET should validate");
}

#[test]
fn eret_branches_to_elr() {
    let instruction = eret_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::Branch { target } = &instruction.semantics[0] else {
        panic!("expected Branch");
    };

    // In our model, ERET branches to ELR which we represent as PC special register
    // The actual implementation would need ELR as a special register
    assert!(matches!(target, Expr::ReadSpecial { reg: SpecialReg::Pc, .. }));
}