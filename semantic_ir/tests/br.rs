/// BR (Branch to Register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/br.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn br_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // target = Xn
    let target = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // BR: BranchTo(target, INDIR)
    let semantics = vec![Stmt::Branch { target }];

    Instruction {
        id: InstructionId {
            mnemonic: "BR".to_string(),
            encoding: Some("Branch to register".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn br_validates() {
    let instruction = br_instruction();

    validate_instruction(&instruction).expect("BR should validate");
}

#[test]
fn br_uses_branch_with_register_target() {
    let instruction = br_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::Branch { target } = &instruction.semantics[0] else {
        panic!("expected Branch");
    };

    // Target should be Xn
    assert!(matches!(target, Expr::ReadReg { reg: Reg::X(2), .. }));
}