/// ADDPT semantics are representable in the IR.
/// Stanford source: arm64_md_clean/addpt.md
/// Stanford (arm64_md_no_nav_clean/addpt.md, FEAT_CPA "add checked pointer"):
///   base = SP/Xn; offset = LSL(Xm, shift); result = base + offset;
///   result = PointerAddCheck(result, base) — top-byte mismatch with base
///   makes the result likely non-canonical; write SP/Xd.
///
/// Example below: n=2, m=3, d=1, shift=2.
use semantic_ir::{
    BinOp, BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, Reg, ShiftKind, Stmt, ValueType, Width, validate_instruction,
};

fn addpt_instruction() -> Instruction {
    // base: SP if n==31 else Xn (example n=2).
    let base = Expr::Select {
        condition: Box::new(Expr::Binary {
            op: BinOp::Eq,
            left: Box::new(Expr::Const {
                value: 2,
                ty: BitWidth::W32,
            }),
            right: Box::new(Expr::Const {
                value: 31,
                ty: BitWidth::W32,
            }),
            ty: ValueType::Bool,
        }),
        when_true: Box::new(Expr::ReadSP {
            width: BitWidth::W64,
        }),
        when_false: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // offset = LSL(Xm, shift) (example m=3, shift=2).
    let offset = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(Expr::ReadReg {
            reg: Reg::X(3),
            width: BitWidth::W64,
        }),
        amount: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        ty: BitWidth::W64,
    };

    // result = base + offset, then PointerAddCheck(result, base).
    let checked = Expr::PointerAddCheck {
        value: Box::new(Expr::Binary {
            op: BinOp::Add,
            left: Box::new(base.clone()),
            right: Box::new(offset),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
        base: Box::new(base),
    };

    // if d == 31 then SP = checked else Xd = checked (example d=1).
    let semantics = vec![Stmt::If {
        condition: Expr::Binary {
            op: BinOp::Eq,
            left: Box::new(Expr::Const {
                value: 1,
                ty: BitWidth::W32,
            }),
            right: Box::new(Expr::Const {
                value: 31,
                ty: BitWidth::W32,
            }),
            ty: ValueType::Bool,
        },
        then_body: vec![Stmt::WriteSP {
            value: checked.clone(),
            width: Width::Fixed(BitWidth::W64),
        }],
        else_body: vec![Stmt::WriteReg {
            reg: Reg::X(1),
            value: checked,
        }],
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDPT".to_string(),
            encoding: Some("Add checked pointer".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "d".to_string(),
                    value: DecodeExpr::UInt("Rd".to_string()),
                },
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
                DecodeStmt::Let {
                    name: "m".to_string(),
                    value: DecodeExpr::UInt("Rm".to_string()),
                },
                DecodeStmt::Let {
                    name: "shift".to_string(),
                    value: DecodeExpr::UInt("imm3".to_string()),
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "addpt.md".to_string(),
            source_sha256:
                "f2f1a817a1ac888cdfffddac70273253ef96fe8b6740c5d946e79aa5c2b63d7f".to_string(),
            clean_sha256:
                "c5e30488f9345205edb534d89a6f49f06e1d2a3e07b5706a9ba3d42ee3b79f57".to_string(),
        },
    }
}

fn find_pointer_check(expr: &Expr) -> Option<(&Box<Expr>, &Box<Expr>)> {
    match expr {
        Expr::PointerAddCheck { value, base } => Some((value, base)),
        Expr::Select {
            condition,
            when_true,
            when_false,
..
        } => find_pointer_check(condition)
            .or_else(|| find_pointer_check(when_true))
            .or_else(|| find_pointer_check(when_false)),
        Expr::Binary { left, right, .. } => {
            find_pointer_check(left).or_else(|| find_pointer_check(right))
        }
        Expr::Concat { high, low, .. } => {
            find_pointer_check(high).or_else(|| find_pointer_check(low))
        }
        Expr::TupleElement { value, .. }
        | Expr::Extract { value, .. }
        | Expr::Unary { value, .. } => find_pointer_check(value),
        _ => None,
    }
}

fn write_value(instruction: &Instruction) -> &Expr {
    let Stmt::If { else_body, .. } = &instruction.semantics[0] else {
        panic!("expected If for conditional destination");
    };
    let Stmt::WriteReg { value, .. } = &else_body[0] else {
        panic!("expected WriteReg");
    };
    value
}

#[test]
fn addpt_validates() {
    let instruction = addpt_instruction();

    validate_instruction(&instruction).expect("ADDPT should validate");
}

#[test]
fn addpt_requires_cpa_feature() {
    let instruction = addpt_instruction();

    assert_eq!(instruction.required_features, vec!["FEAT_CPA".to_string()]);

    let decode = instruction.decode.expect("ADDPT needs decode bindings");
    assert_eq!(decode.statements.len(), 4);
}

#[test]
fn addpt_adds_shifted_offset() {
    let instruction = addpt_instruction();
    let value = write_value(&instruction);

    let (inner, base) = find_pointer_check(value).expect("result must be checked");
    let Expr::Binary {
        op: BinOp::Add,
        left,
        right,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    } = inner.as_ref()
    else {
        panic!("checked value must be a 64-bit add");
    };

    // Left operand is the base register value (SP/Xn select).
    assert!(matches!(left.as_ref(), Expr::Select { .. }));

    // Right operand = LSL(Xm, shift).
    let Expr::Shift {
        kind: ShiftKind::Lsl,
        value: shifted,
        amount,
        ty: BitWidth::W64,
    } = right.as_ref()
    else {
        panic!("offset must be a 64-bit logical left shift");
    };
    assert!(matches!(
        shifted.as_ref(),
        Expr::ReadReg { reg: Reg::X(3), .. }
    ));
    assert!(matches!(amount.as_ref(), Expr::Const { value: 2, .. }));

    // Check compares against the same base.
    assert!(matches!(base.as_ref(), Expr::Select { .. }));
}

#[test]
fn addpt_check_preserves_64_bits() {
    let instruction = addpt_instruction();
    let value = write_value(&instruction);

    // The stored value itself must be the PointerAddCheck node.
    assert!(matches!(value, Expr::PointerAddCheck { .. }));
}

#[test]
fn addpt_writes_sp_or_register() {
    let instruction = addpt_instruction();

    let Stmt::If {
        condition,
        then_body,
        else_body,
    } = &instruction.semantics[0]
    else {
        panic!("expected If for conditional destination");
    };

    assert!(matches!(condition, Expr::Binary { op: BinOp::Eq, .. }));
    assert!(matches!(then_body[0], Stmt::WriteSP { .. }));
    assert!(matches!(
        else_body[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}
