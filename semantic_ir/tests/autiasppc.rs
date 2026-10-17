/// AUTIASPPC semantics are representable in the IR.
/// Stanford source: arm64_md_clean/autiasppc_imm.md
/// Stanford (arm64_md_no_nav_clean/autiasppc_imm.md):
///   AuthIA2(X30, SP, PC64() - offset, is_combined=false)
///   Two-modifier instruction address authentication (key A).
///   Requires FEAT_PAuth_LR. offset = ZeroExtend(imm16::'00').
use semantic_ir::{
    BitWidth, BinOp, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn autiasppc_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(30),
        width: BitWidth::W64,
    };

    let modifier1 = Expr::ReadSP {
        width: BitWidth::W64,
    };

    // pac_addr = PC64() - offset
    let pc = Expr::ReadSpecial {
        reg: SpecialReg::Pc64,
        width: BitWidth::W64,
    };

    let offset = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };

    let pac_addr = Expr::Binary {
        op: BinOp::Sub,
        left: Box::new(pc),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let auth_combined = Expr::Bool(false);

    let authenticated = Expr::AuthIA2 {
        address: Box::new(address),
        modifier1: Box::new(modifier1),
        modifier2: Box::new(pac_addr),
        auth_combined: Box::new(auth_combined),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "AUTIASPPC".to_string(),
            encoding: Some("Authenticate return address (X30), modifiers SP + PC-offset".to_string()),
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
                    name: "offset".to_string(),
                    value: DecodeExpr::Immediate(0),
                },
            ],
        }),
        semantics: vec![Stmt::WriteReg {
            reg: Reg::X(30),
            value: authenticated,
        }],
        provenance: Provenance {
            source_path: "autiasppc_imm.md".to_string(),
            source_sha256:
                "0d27e663a9984e6aadc2e381969eb4e85637988f45c966455957fa1482f1fdf3".to_string(),
            clean_sha256:
                "ba38aa15436aac0abcafb0149cb054f03003cacbc383f5b6f444c2e056dac315".to_string(),
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
        Expr::Binary { left, right, .. } => find_authia2_expr(left).or_else(|| find_authia2_expr(right)),
        _ => None,
    }
}

#[test]
fn autiasppc_validates() {
    let instruction = autiasppc_instruction();
    validate_instruction(&instruction).expect("AUTIASPPC should validate");
}

#[test]
fn autiasppc_requires_pauth_lr_feature() {
    let instruction = autiasppc_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_PAuth_LR".to_string()]);
}

#[test]
fn autiasppc_uses_authia2_with_sp_and_pc_minus_offset() {
    let instruction = autiasppc_instruction();
    let (address, mod1, mod2, auth_combined) = find_authia2(&instruction.semantics[0])
        .expect("semantics must contain AuthIA2");

    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(30), .. }));
    assert!(matches!(mod1.as_ref(), Expr::ReadSP { width: BitWidth::W64 }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(false)));

    // mod2 = PC64() - offset
    let Expr::Binary { op: BinOp::Sub, left, right, ty: ValueType::Bits(Width::Fixed(BitWidth::W64)) } = mod2.as_ref()
    else {
        panic!("mod2 must be PC64 - offset");
    };
    assert!(matches!(left.as_ref(), Expr::ReadSpecial { reg: SpecialReg::Pc64, width: BitWidth::W64 }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 0, ty: BitWidth::W64 }));
}