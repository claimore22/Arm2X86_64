/// AUTIA171615 semantics are representable in the IR.
/// Stanford source: arm64_md_clean/autia171615.md
/// Stanford (arm64_md_no_nav_clean/autia171615.md):
///   AuthIA2(X17, X16, X15, is_combined=false) — two-modifier
///   instruction address authentication (key A). Requires FEAT_PAuth_LR.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn autia171615_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(17),
        width: BitWidth::W64,
    };

    let modifier1 = Expr::ReadReg {
        reg: Reg::X(16),
        width: BitWidth::W64,
    };

    let modifier2 = Expr::ReadReg {
        reg: Reg::X(15),
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
            mnemonic: "AUTIA171615".to_string(),
            encoding: Some("Authenticate instruction address (X17), modifiers X16+X15".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "d".to_string(),
                    value: DecodeExpr::Immediate(17),
                },
            ],
        }),
        semantics: vec![Stmt::WriteReg {
            reg: Reg::X(17),
            value: authenticated,
        }],
        provenance: Provenance {
            source_path: "autia171615.md".to_string(),
            source_sha256:
                "f573d79c91db6b2d5709dbb9a47709aa2f3da9e3ed341e04f22a175894447061".to_string(),
            clean_sha256:
                "2e5415875781341a9da405859f9d55ac2012fa21f499b7ff78c4a476c21784df".to_string(),
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
fn autia171615_validates() {
    let instruction = autia171615_instruction();
    validate_instruction(&instruction).expect("AUTIA171615 should validate");
}

#[test]
fn autia171615_requires_pauth_lr_feature() {
    let instruction = autia171615_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_PAuth_LR".to_string()]);
}

#[test]
fn autia171615_uses_authia2_with_correct_registers() {
    let instruction = autia171615_instruction();
    let (address, mod1, mod2, auth_combined) = find_authia2(&instruction.semantics[0])
        .expect("semantics must contain AuthIA2");

    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(17), .. }));
    assert!(matches!(mod1.as_ref(), Expr::ReadReg { reg: Reg::X(16), .. }));
    assert!(matches!(mod2.as_ref(), Expr::ReadReg { reg: Reg::X(15), .. }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(false)));
}