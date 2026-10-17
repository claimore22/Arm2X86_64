/// RETAA, RETAB (Return from subroutine, with pointer authentication)
/// Stanford source: arm64_md_clean/reta.md
/// semantics are representable in the IR.
///
/// branch to AuthIA/AuthIB(X(30), SP, auth_combined=true). Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn retaa_instruction(key_a: bool) -> Instruction {
    let lr = Expr::ReadReg {
        reg: Reg::X(30),
        width: BitWidth::W64,
    };
    let sp = Expr::ReadSP {
        width: BitWidth::W64,
    };

    let target = if key_a {
        Expr::AuthIA {
            address: Box::new(lr),
            modifier: Box::new(sp),
            auth_combined: Box::new(Expr::Bool(true)),
        }
    } else {
        Expr::AuthIB {
            address: Box::new(lr),
            modifier: Box::new(sp),
            auth_combined: Box::new(Expr::Bool(true)),
        }
    };

    let semantics = vec![Stmt::Branch { target }];

    Instruction {
        id: InstructionId {
            mnemonic: if key_a { "RETAA" } else { "RETAB" }.to_string(),
            encoding: Some("Return from subroutine with pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "reta.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn retaa_validates() {
    validate_instruction(&retaa_instruction(true)).expect("RETAA should validate");
}

#[test]
fn retab_validates() {
    validate_instruction(&retaa_instruction(false)).expect("RETAB should validate");
}

#[test]
fn retaa_uses_key_a() {
    let instruction = retaa_instruction(true);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Branch {
            target: Expr::AuthIA { .. }
        }
    ));
}

#[test]
fn retab_uses_key_b() {
    let instruction = retaa_instruction(false);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Branch {
            target: Expr::AuthIB { .. }
        }
    ));
}
