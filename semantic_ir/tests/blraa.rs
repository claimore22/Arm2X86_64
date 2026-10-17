/// BLRAA family semantics are representable in the IR.
/// Stanford source: arm64_md_clean/blra.md
/// Stanford (arm64_md_no_nav_clean/blra.md):
///   target = Xn; modifier = SP if source_is_sp else Xm;
///   if use_key_a { target = AuthIA(target, modifier, TRUE) }
///   else { target = AuthIB(target, modifier, TRUE) };
///   X30 = PC + 4; BranchTo(target, INDCALL).
/// Variants:
///   BLRAA  : key A, modifier Xm
///   BLRAAZ : key A, modifier zero
///   BLRAB  : key B, modifier Xm
///   BLRABZ : key B, modifier zero
/// No IR extension: reuses AuthIA/AuthIB + Branch + WriteReg.
/// GCS/BTypeNext omitted like BLR (no IR node).
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn pc_plus_4() -> Expr {
    Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc,
            width: BitWidth::W64,
        }),
        right: Box::new(Expr::Const {
            value: 4,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    }
}

fn blraa_instruction() -> Instruction {
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
            mnemonic: "BLRAA".to_string(),
            encoding: Some("Branch with link to register, with pointer authentication".to_string()),
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
        semantics: vec![
            Stmt::WriteReg {
                reg: Reg::X(30),
                value: pc_plus_4(),
            },
            Stmt::Branch {
                target: authenticated,
            },
        ],
        provenance: Provenance {
            source_path: "blra.md".to_string(),
            source_sha256:
                "1b732dbd68eb0e7bc21aec3749800b1541fcbebece7da239908cef2453428e0f".to_string(),
            clean_sha256:
                "942da6f119eec7fdcbb725fa558667bc4515c497714a972f56d24e404775a059".to_string(),
        },
    }
}

fn blraaz_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };
    let authenticated = Expr::AuthIA {
        address: Box::new(address),
        modifier: Box::new(modifier),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BLRAAZ".to_string(),
            encoding: Some("Branch with link to register, with pointer authentication".to_string()),
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
        semantics: vec![
            Stmt::WriteReg {
                reg: Reg::X(30),
                value: pc_plus_4(),
            },
            Stmt::Branch {
                target: authenticated,
            },
        ],
        provenance: Provenance {
            source_path: "blra.md".to_string(),
            source_sha256:
                "1b732dbd68eb0e7bc21aec3749800b1541fcbebece7da239908cef2453428e0f".to_string(),
            clean_sha256:
                "942da6f119eec7fdcbb725fa558667bc4515c497714a972f56d24e404775a059".to_string(),
        },
    }
}

fn blrab_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };
    let authenticated = Expr::AuthIB {
        address: Box::new(address),
        modifier: Box::new(modifier),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BLRAB".to_string(),
            encoding: Some("Branch with link to register, with pointer authentication".to_string()),
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
        semantics: vec![
            Stmt::WriteReg {
                reg: Reg::X(30),
                value: pc_plus_4(),
            },
            Stmt::Branch {
                target: authenticated,
            },
        ],
        provenance: Provenance {
            source_path: "blra.md".to_string(),
            source_sha256:
                "1b732dbd68eb0e7bc21aec3749800b1541fcbebece7da239908cef2453428e0f".to_string(),
            clean_sha256:
                "942da6f119eec7fdcbb725fa558667bc4515c497714a972f56d24e404775a059".to_string(),
        },
    }
}

fn blrabz_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };
    let authenticated = Expr::AuthIB {
        address: Box::new(address),
        modifier: Box::new(modifier),
        auth_combined: Box::new(Expr::Bool(true)),
    };
    Instruction {
        id: InstructionId {
            mnemonic: "BLRABZ".to_string(),
            encoding: Some("Branch with link to register, with pointer authentication".to_string()),
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
        semantics: vec![
            Stmt::WriteReg {
                reg: Reg::X(30),
                value: pc_plus_4(),
            },
            Stmt::Branch {
                target: authenticated,
            },
        ],
        provenance: Provenance {
            source_path: "blra.md".to_string(),
            source_sha256:
                "1b732dbd68eb0e7bc21aec3749800b1541fcbebece7da239908cef2453428e0f".to_string(),
            clean_sha256:
                "942da6f119eec7fdcbb725fa558667bc4515c497714a972f56d24e404775a059".to_string(),
        },
    }
}

#[test]
fn blraa_validates() {
    validate_instruction(&blraa_instruction()).expect("BLRAA should validate");
}

#[test]
fn blraa_requires_pauth_and_links_and_auth_branches() {
    let ins = blraa_instruction();
    assert_eq!(ins.required_features, vec!["FEAT_PAuth".to_string()]);
    assert_eq!(ins.semantics.len(), 2);
    let Stmt::WriteReg { reg, .. } = &ins.semantics[0] else {
        panic!("expected WriteReg X30");
    };
    assert_eq!(*reg, Reg::X(30));
    let Stmt::Branch { target } = &ins.semantics[1] else {
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
fn blraaz_validates_zero_modifier() {
    let ins = blraaz_instruction();
    validate_instruction(&ins).expect("BLRAAZ should validate");
    let Stmt::Branch { target } = &ins.semantics[1] else {
        panic!("expected Branch");
    };
    let Expr::AuthIA { modifier, .. } = target else {
        panic!("expected AuthIA");
    };
    assert!(matches!(modifier.as_ref(), Expr::Const { value: 0, .. }));
}

#[test]
fn blrab_validates_key_b() {
    let ins = blrab_instruction();
    validate_instruction(&ins).expect("BLRAB should validate");
    let Stmt::Branch { target } = &ins.semantics[1] else {
        panic!("expected Branch");
    };
    assert!(matches!(target, Expr::AuthIB { .. }));
}

#[test]
fn blrabz_validates_key_b_zero() {
    let ins = blrabz_instruction();
    validate_instruction(&ins).expect("BLRABZ should validate");
    let Stmt::Branch { target } = &ins.semantics[1] else {
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
