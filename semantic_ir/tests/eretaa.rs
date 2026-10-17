/// ERETAA, ERETAB (Exception return with pointer authentication) semantics are
/// Stanford source: arm64_md_clean/ereta.md
/// representable in the IR.
///
/// target = AuthIA/AuthIB(ELR_ELx, SP, auth_then_branch=true), then
/// AArch64_ExceptionReturn(target, SPSR_ELx). Modeled as a Branch to the
/// authenticated address using the existing `Expr::AuthIA`/`Expr::AuthIB`
/// nodes (key A for ERETAA, key B for ERETAB). Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, SpecialReg, Stmt,
    validate_instruction,
};

fn eretaa_instruction() -> Instruction {
    // target = ELR_ELx (modeled as PC special register, same as ERET)
    let target = Expr::ReadSpecial {
        reg: SpecialReg::Pc,
        width: BitWidth::W64,
    };

    // modifier = SP
    let modifier = Expr::ReadSP {
        width: BitWidth::W64,
    };

    let auth_then_branch = Expr::Bool(true);

    // ERETAA uses key A: target = AuthIA(target, modifier, auth_then_branch)
    let authenticated = Expr::AuthIA {
        address: Box::new(target),
        modifier: Box::new(modifier),
        auth_combined: Box::new(auth_then_branch),
    };

    let semantics = vec![Stmt::Branch {
        target: authenticated,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ERETAA".to_string(),
            encoding: Some("Exception return, pointer auth key A".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ereta.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn eretab_instruction() -> Instruction {
    let target = Expr::ReadSpecial {
        reg: SpecialReg::Pc,
        width: BitWidth::W64,
    };

    let modifier = Expr::ReadSP {
        width: BitWidth::W64,
    };

    let auth_then_branch = Expr::Bool(true);

    // ERETAB uses key B: target = AuthIB(target, modifier, auth_then_branch)
    let authenticated = Expr::AuthIB {
        address: Box::new(target),
        modifier: Box::new(modifier),
        auth_combined: Box::new(auth_then_branch),
    };

    let semantics = vec![Stmt::Branch {
        target: authenticated,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ERETAB".to_string(),
            encoding: Some("Exception return, pointer auth key B".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ereta.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eretaa_validates() {
    let instruction = eretaa_instruction();
    validate_instruction(&instruction).expect("ERETAA should validate");
}

#[test]
fn eretab_validates() {
    let instruction = eretab_instruction();
    validate_instruction(&instruction).expect("ERETAB should validate");
}

#[test]
fn eretaa_uses_key_a() {
    let instruction = eretaa_instruction();

    let Stmt::Branch { target } = &instruction.semantics[0] else {
        panic!("expected Branch");
    };

    assert!(matches!(target, Expr::AuthIA { .. }));
}

#[test]
fn eretab_uses_key_b() {
    let instruction = eretab_instruction();

    let Stmt::Branch { target } = &instruction.semantics[0] else {
        panic!("expected Branch");
    };

    assert!(matches!(target, Expr::AuthIB { .. }));
}

#[test]
fn ereta_requires_pauth() {
    assert!(eretaa_instruction()
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
    assert!(eretab_instruction()
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
}
