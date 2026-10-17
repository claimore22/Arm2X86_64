/// AUTIASPPCR semantics are representable in the IR.
/// Stanford source: arm64_md_clean/autiasppcr.md
/// Stanford (arm64_md_no_nav_clean/autiasppcr.md):
///   AuthIA2(X30, SP, Xn, is_combined=false)
///   Two-modifier instruction address authentication (key A).
///   Requires FEAT_PAuth_LR.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn autiasppcr_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(30),
        width: BitWidth::W64,
    };

    let modifier1 = Expr::ReadSP {
        width: BitWidth::W64,
    };

    let modifier2 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let auth_combined = Expr::Bool(false);

    let authenticated = Expr::AuthIA2 {
        address: Box::new(address),
        modifier1: Box::new(modifier1),
        modifier2: Box::new(modifier2),
        auth_combined: Box::new(auth_combined),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "AUTIASPPCR".to_string(),
            encoding: Some("Authenticate return address (X30), modifiers SP + Xn".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "d".to_string(),
                    value: DecodeExpr::Immediate(30),
                },
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
            ],
        }),
        semantics: vec![Stmt::WriteReg {
            reg: Reg::X(30),
            value: authenticated,
        }],
        provenance: Provenance {
            source_path: "autiasppcr.md".to_string(),
            source_sha256:
                "380d0bd9c73d7b76fb2f019a9858bd77df169cc8a194a25f6cec1deab355a074".to_string(),
            clean_sha256:
                "57d91a6803262b0436d6f3b4af96e1f09b53ba2fa7ac9dada1eb59050e1bbd04".to_string(),
        },
    }
}

fn find_authia2(stmt: &Stmt) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match stmt {
        Stmt::WriteReg { value, .. } => find_authia2_expr(value),
        _ => None,
    }
}

fn find_authia2_expr(expr: &Expr) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match expr {
        Expr::AuthIA2 { address, modifier1, modifier2, auth_combined } => Some((address, modifier1, modifier2, auth_combined)),
        _ => None,
    }
}

#[test]
fn autiasppcr_validates() {
    let instruction = autiasppcr_instruction();
    validate_instruction(&instruction).expect("AUTIASPPCR should validate");
}

#[test]
fn autiasppcr_requires_pauth_lr_feature() {
    let instruction = autiasppcr_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_PAuth_LR".to_string()]);
}

#[test]
fn autiasppcr_uses_authia2_with_sp_and_xn() {
    let instruction = autiasppcr_instruction();
    let (address, mod1, mod2, auth_combined) = find_authia2(&instruction.semantics[0])
        .expect("semantics must contain AuthIA2");

    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(30), .. }));
    assert!(matches!(mod1.as_ref(), Expr::ReadSP { width: BitWidth::W64 }));
    assert!(matches!(mod2.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(false)));
}