/// AUTIB171615 semantics are representable in the IR.
/// Stanford source: arm64_md_clean/autib171615.md
/// Stanford (arm64_md_no_nav_clean/autib171615.md):
///   AuthIB2(X17, X16, X15, is_combined=false) — two-modifier
///   instruction address authentication (key B). Requires FEAT_PAuth_LR.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn autib171615_instruction() -> Instruction {
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

    let authenticated = Expr::AuthIB2 {
        address: Box::new(address),
        modifier1: Box::new(modifier1),
        modifier2: Box::new(modifier2),
        auth_combined: Box::new(auth_combined),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "AUTIB171615".to_string(),
            encoding: Some("Authenticate instruction address (X17), modifiers X16+X15, key B".to_string()),
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
            source_path: "autib171615.md".to_string(),
            source_sha256:
                "517e8119c621404c2866345230aa2067762525301c53240c347a0fc5ba416a98".to_string(),
            clean_sha256:
                "b045d0fbdf4911e94ffc2427ddf2658bfb00c8fbae41fd39d3ca672e58f58a38".to_string(),
        },
    }
}

fn find_authib2(stmt: &Stmt) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match stmt {
        Stmt::WriteReg { value, .. } => find_authib2_expr(value),
        _ => None,
    }
}

fn find_authib2_expr(expr: &Expr) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match expr {
        Expr::AuthIB2 { address, modifier1, modifier2, auth_combined } => Some((address, modifier1, modifier2, auth_combined)),
        _ => None,
    }
}

#[test]
fn autib171615_validates() {
    let instruction = autib171615_instruction();
    validate_instruction(&instruction).expect("AUTIB171615 should validate");
}

#[test]
fn autib171615_requires_pauth_lr_feature() {
    let instruction = autib171615_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_PAuth_LR".to_string()]);
}

#[test]
fn autib171615_uses_authib2_with_correct_registers() {
    let instruction = autib171615_instruction();
    let (address, mod1, mod2, auth_combined) = find_authib2(&instruction.semantics[0])
        .expect("semantics must contain AuthIB2");

    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(17), .. }));
    assert!(matches!(mod1.as_ref(), Expr::ReadReg { reg: Reg::X(16), .. }));
    assert!(matches!(mod2.as_ref(), Expr::ReadReg { reg: Reg::X(15), .. }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(false)));
}