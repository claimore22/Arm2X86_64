/// RET (Return from Subroutine) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ret.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ret_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // target = Xn (typically X30)
    let target = Expr::ReadReg {
        reg: Reg::X(30),
        width: BitWidth::W64,
    };

    // RET: BranchTo(target, RET)
    let semantics = vec![Stmt::Branch { target }];

    Instruction {
        id: InstructionId {
            mnemonic: "RET".to_string(),
            encoding: Some("Return from subroutine".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ret.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ret_validates() {
    let instruction = ret_instruction();

    validate_instruction(&instruction).expect("RET should validate");
}

#[test]
fn ret_branches_to_link_register() {
    let instruction = ret_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::Branch { target } = &instruction.semantics[0] else {
        panic!("expected Branch");
    };

    // Target should be X30 (link register)
    assert!(matches!(target, Expr::ReadReg { reg: Reg::X(30), .. }));
}