/// RETAASPPC, RETABSPPC and RETAASPPCR, RETABSPPCR (return from subroutine
/// Stanford source: arm64_md_clean/retasppc_imm.md
/// with enhanced pointer authentication) semantics are representable in the IR.
///
/// Branch to AuthIA2/AuthIB2(X(30), SP, PC/imm, combined=true). Requires
/// FEAT_PAuth_LR (FEAT_PAuth plus the second modifier when PACM=1).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn retaasppc_instruction(key_a: bool) -> Instruction {
    let lr = Expr::ReadReg {
        reg: Reg::X(30),
        width: BitWidth::W64,
    };
    let sp = Expr::ReadSP {
        width: BitWidth::W64,
    };
    // second modifier: immediate offset, or PC for the register variant
    let modifier2 = Expr::ReadSpecial {
        reg: SpecialReg::Pc64,
        width: BitWidth::W64,
    };

    let target = if key_a {
        Expr::AuthIA2 {
            address: Box::new(lr),
            modifier1: Box::new(sp),
            modifier2: Box::new(modifier2),
            auth_combined: Box::new(Expr::Bool(true)),
        }
    } else {
        Expr::AuthIB2 {
            address: Box::new(lr),
            modifier1: Box::new(sp),
            modifier2: Box::new(modifier2),
            auth_combined: Box::new(Expr::Bool(true)),
        }
    };

    let semantics = vec![Stmt::Branch { target }];

    Instruction {
        id: InstructionId {
            mnemonic: if key_a { "RETAASPPC" } else { "RETABSPPC" }.to_string(),
            encoding: Some("Return with enhanced pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "retasppc_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn retaasppc_validates() {
    validate_instruction(&retaasppc_instruction(true)).expect("RETAASPPC should validate");
}

#[test]
fn retabsppc_validates() {
    validate_instruction(&retaasppc_instruction(false)).expect("RETABSPPC should validate");
}

#[test]
fn retaasppc_uses_two_modifier_auth() {
    let instruction = retaasppc_instruction(true);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Branch {
            target: Expr::AuthIA2 { .. }
        }
    ));
}

#[test]
fn retabsppc_uses_two_modifier_auth() {
    let instruction = retaasppc_instruction(false);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Branch {
            target: Expr::AuthIB2 { .. }
        }
    ));
}
