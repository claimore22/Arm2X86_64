/// AUTIBSPPCR semantics are representable in the IR.
/// Stanford source: arm64_md_clean/autibsppcr.md
/// Stanford (arm64_md_no_nav_clean/autibsppcr.md):
///   AuthIB2(X30, SP, Xn, is_combined=false)
///   Two-modifier instruction address authentication (key B).
///   Requires FEAT_PAuth_LR.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn autibsppcr_instruction() -> Instruction {
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

    let authenticated = Expr::AuthIB2 {
        address: Box::new(address),
        modifier1: Box::new(modifier1),
        modifier2: Box::new(modifier2),
        auth_combined: Box::new(auth_combined),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "AUTIBSPPCR".to_string(),
            encoding: Some("Authenticate return address (X30), modifiers SP + Xn, key B".to_string()),
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
            source_path: "autibsppcr.md".to_string(),
            source_sha256:
                "157b04cb9bec5678b32243d3d61cab135fa1f0ce66d8fc98534f3ae4e4efa8f9".to_string(),
            clean_sha256:
                "d451d0c101e5c4042c52260195e68eb9d282803f6db2aa3fcd1d006acf7c2776".to_string(),
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
fn autibsppcr_validates() {
    let instruction = autibsppcr_instruction();
    validate_instruction(&instruction).expect("AUTIBSPPCR should validate");
}

#[test]
fn autibsppcr_requires_pauth_lr_feature() {
    let instruction = autibsppcr_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_PAuth_LR".to_string()]);
}

#[test]
fn autibsppcr_uses_authib2_with_sp_and_xn() {
    let instruction = autibsppcr_instruction();
    let (address, mod1, mod2, auth_combined) = find_authib2(&instruction.semantics[0])
        .expect("semantics must contain AuthIB2");

    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(30), .. }));
    assert!(matches!(mod1.as_ref(), Expr::ReadSP { width: BitWidth::W64 }));
    assert!(matches!(mod2.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(false)));
}