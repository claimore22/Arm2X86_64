/// BRAA family semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bra.md
/// Stanford (arm64_md_no_nav_clean/bra.md):
///   target = Xn; modifier = SP if source_is_sp else Xm;
///   if use_key_a { target = AuthIA(target, modifier, TRUE) }
///   else { target = AuthIB(target, modifier, TRUE) };
///   BranchTo(target, INDIR).
/// Variants:
///   BRAA  : key A, modifier Xm
///   BRAAZ : key A, modifier zero
///   BRAB  : key B, modifier Xm
///   BRABZ : key B, modifier zero
/// No IR extension: reuses AuthIA/AuthIB + Branch (like BR + Auth).
/// BTypeNext omitted like BR (no IR node).
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, validate_instruction,
};

fn braa_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };
    let authenticated = Expr::AuthIA {
        address: Box::new(address),
        modifier: Box::new(modifier),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BRAA".to_string(),
            encoding: Some("Branch to register, with pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
                DecodeStmt::Let {
                    name: "m".to_string(),
                    value: DecodeExpr::UInt("Rm".to_string()),
                },
                DecodeStmt::Let {
                    name: "use_key_a".to_string(),
                    value: DecodeExpr::Identifier("M==0".to_string()),
                },
                DecodeStmt::Let {
                    name: "source_is_sp".to_string(),
                    value: DecodeExpr::Identifier("Z".to_string()),
                },
            ],
        }),
        semantics: vec![Stmt::Branch {
            target: authenticated,
        }],
        provenance: Provenance {
            source_path: "bra.md".to_string(),
            source_sha256:
                "70621f6873388358a901fcae620d1614dafcf5e657ef1244a71a61b288f3a415".to_string(),
            clean_sha256:
                "af06440aa275b506bfa617776aa4db121d0dd568dadde0c12a10304ec2782972".to_string(),
        },
    }
}

fn braaz_instruction() -> Instruction {
    let authenticated = Expr::AuthIA {
        address: Box::new(Expr::ReadReg {
            reg: Reg::X(1),
            width: BitWidth::W64,
        }),
        modifier: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W64,
        }),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BRAAZ".to_string(),
            encoding: Some("Branch to register, with pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
                DecodeStmt::Let {
                    name: "modifier".to_string(),
                    value: DecodeExpr::Immediate(0),
                },
            ],
        }),
        semantics: vec![Stmt::Branch {
            target: authenticated,
        }],
        provenance: Provenance {
            source_path: "bra.md".to_string(),
            source_sha256:
                "70621f6873388358a901fcae620d1614dafcf5e657ef1244a71a61b288f3a415".to_string(),
            clean_sha256:
                "af06440aa275b506bfa617776aa4db121d0dd568dadde0c12a10304ec2782972".to_string(),
        },
    }
}

fn brab_instruction() -> Instruction {
    let authenticated = Expr::AuthIB {
        address: Box::new(Expr::ReadReg {
            reg: Reg::X(1),
            width: BitWidth::W64,
        }),
        modifier: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BRAB".to_string(),
            encoding: Some("Branch to register, with pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
                DecodeStmt::Let {
                    name: "m".to_string(),
                    value: DecodeExpr::UInt("Rm".to_string()),
                },
            ],
        }),
        semantics: vec![Stmt::Branch {
            target: authenticated,
        }],
        provenance: Provenance {
            source_path: "bra.md".to_string(),
            source_sha256:
                "70621f6873388358a901fcae620d1614dafcf5e657ef1244a71a61b288f3a415".to_string(),
            clean_sha256:
                "af06440aa275b506bfa617776aa4db121d0dd568dadde0c12a10304ec2782972".to_string(),
        },
    }
}

fn brabz_instruction() -> Instruction {
    let authenticated = Expr::AuthIB {
        address: Box::new(Expr::ReadReg {
            reg: Reg::X(1),
            width: BitWidth::W64,
        }),
        modifier: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W64,
        }),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BRABZ".to_string(),
            encoding: Some("Branch to register, with pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
                DecodeStmt::Let {
                    name: "modifier".to_string(),
                    value: DecodeExpr::Immediate(0),
                },
            ],
        }),
        semantics: vec![Stmt::Branch {
            target: authenticated,
        }],
        provenance: Provenance {
            source_path: "bra.md".to_string(),
            source_sha256:
                "70621f6873388358a901fcae620d1614dafcf5e657ef1244a71a61b288f3a415".to_string(),
            clean_sha256:
                "af06440aa275b506bfa617776aa4db121d0dd568dadde0c12a10304ec2782972".to_string(),
        },
    }
}

#[test]
fn braa_validates() {
    validate_instruction(&braa_instruction()).expect("BRAA should validate");
}

#[test]
fn braa_requires_pauth_and_auth_branches() {
    let ins = braa_instruction();
    assert_eq!(ins.required_features, vec!["FEAT_PAuth".to_string()]);
    assert_eq!(ins.semantics.len(), 1);
    let Stmt::Branch { target } = &ins.semantics[0] else {
        panic!("expected Branch");
    };
    let Expr::AuthIA {
        address,
        modifier,
        auth_combined,
    } = target
    else {
        panic!("expected AuthIA");
    };
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    assert!(matches!(modifier.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(true)));
}

#[test]
fn braaz_validates_zero_modifier() {
    let ins = braaz_instruction();
    validate_instruction(&ins).expect("BRAAZ should validate");
    let Stmt::Branch { target } = &ins.semantics[0] else {
        panic!("expected Branch");
    };
    let Expr::AuthIA { modifier, .. } = target else {
        panic!("expected AuthIA");
    };
    assert!(matches!(modifier.as_ref(), Expr::Const { value: 0, .. }));
}

#[test]
fn brab_validates_key_b() {
    let ins = brab_instruction();
    validate_instruction(&ins).expect("BRAB should validate");
    let Stmt::Branch { target } = &ins.semantics[0] else {
        panic!("expected Branch");
    };
    assert!(matches!(target, Expr::AuthIB { .. }));
}

#[test]
fn brabz_validates_key_b_zero() {
    let ins = brabz_instruction();
    validate_instruction(&ins).expect("BRABZ should validate");
    let Stmt::Branch { target } = &ins.semantics[0] else {
        panic!("expected Branch");
    };
    let Expr::AuthIB {
        modifier,
        auth_combined,
..
    } = target
    else {
        panic!("expected AuthIB");
    };
    assert!(matches!(modifier.as_ref(), Expr::Const { value: 0, .. }));
    assert!(matches!(auth_combined.as_ref(), Expr::Bool(true)));
}
