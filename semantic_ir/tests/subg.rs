/// SUBG (Subtract with tag) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/subg.md
/// Stanford (arm64_md_no_nav_clean/subg.md):
///   operand1 = SP/Xn; start_tag = AllocationTagFromAddress(operand1)
///   (bits 59:56); rtag = ChooseNonExcludedTagOrZero(start_tag, imm4,
///   GCR_EL1.Exclude); (result,-) = AddWithCarry64(operand1,
///   NOT(LSL(ZeroExtend64(imm6), LOG2_TAG_GRANULE)), '1');
///   result = AddressWithAllocationTag(result, rtag); write SP/Xd.
///
/// Example below: n=2, d=1, imm6=3 (offset 48), imm4 tag offset=5.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, ShiftKind, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn subg_instruction() -> Instruction {
    // operand1: SP if n==31 else Xn (example n=2).
    let n_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 2,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };

    let operand1 = Expr::Select {
        condition: Box::new(n_is_sp),
        when_true: Box::new(Expr::ReadSP {
            width: BitWidth::W64,
        }),
        when_false: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // start_tag = AllocationTagFromAddress(operand1) = operand1<59:56>.
    let start_tag = Expr::Extract {
        value: Box::new(operand1.clone()),
        lsb: 56,
        width: BitWidth::W4,
    };

    // rtag = ChooseNonExcludedTagOrZero(start_tag, tag_offset, GCR_EL1).
    let rtag = Expr::ChooseTag {
        start_tag: Box::new(start_tag),
        tag_offset: Box::new(Expr::Const {
            value: 5,
            ty: BitWidth::W4,
        }),
        gcr: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::GcrEl1,
            width: BitWidth::W64,
        }),
    };

    // offset = LSL(ZeroExtend{64}(imm6), LOG2_TAG_GRANULE=4).
    let offset = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(Expr::ZeroExtend {
            value: Box::new(Expr::Const {
                value: 3,
                ty: BitWidth::new(6).unwrap(),
            }),
            to: BitWidth::W64,
        }),
        amount: Box::new(Expr::Const {
            value: 4,
            ty: BitWidth::W32,
        }),
        ty: BitWidth::W64,
    };

    // SUBG subtracts: AddWithCarry(operand1, NOT(offset), '1').
    // NOT(offset) is a 64-bit bitwise inversion.
    let not_offset = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let add_with_carry = Expr::AddWithCarry {
        left: Box::new(operand1.clone()),
        right: Box::new(not_offset),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    // (result, -) = AddWithCarry(...).
    let result = Expr::TupleElement {
        value: Box::new(add_with_carry),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // result = AddressWithAllocationTag(result, rtag):
    // result<63:60> : rtag : result<55:0>.
    let low = Expr::Extract {
        value: Box::new(result.clone()),
        lsb: 0,
        width: BitWidth::new(56).unwrap(),
    };
    let inner = Expr::Concat {
        high: Box::new(rtag),
        low: Box::new(low),
        width: BitWidth::new(60).unwrap(),
    };
    let tagged = Expr::Concat {
        high: Box::new(Expr::Extract {
            value: Box::new(result),
            lsb: 60,
            width: BitWidth::W4,
        }),
        low: Box::new(inner),
        width: BitWidth::W64,
    };

    // if d == 31 then SP = tagged else Xd = tagged (example d=1).
    let dest_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 1,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };

    let semantics = vec![Stmt::If {
        condition: dest_is_sp,
        then_body: vec![Stmt::WriteSP {
            value: tagged.clone(),
            width: Width::Fixed(BitWidth::W64),
        }],
        else_body: vec![Stmt::WriteReg {
            reg: Reg::X(1),
            value: tagged,
        }],
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SUBG".to_string(),
            encoding: Some("Subtract with tag".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
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
                    name: "tag_offset".to_string(),
                    value: DecodeExpr::UInt("imm4".to_string()),
                },
                DecodeStmt::Let {
                    name: "offset".to_string(),
                    value: DecodeExpr::ShiftLeft {
                        value: Box::new(DecodeExpr::UInt("imm6".to_string())),
                        amount: 4,
                    },
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "subg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// Recursive structural search helpers so tests assert shapes, not positions.
fn find_choose_tag(expr: &Expr) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match expr {
        Expr::ChooseTag {
            start_tag,
            tag_offset,
            gcr,
        } => Some((start_tag, tag_offset, gcr)),
        Expr::Binary { left, right, .. } => {
            find_choose_tag(left).or_else(|| find_choose_tag(right))
        }
        Expr::Unary { value, .. } => find_choose_tag(value),
        Expr::Shift { value, amount, .. } => {
            find_choose_tag(value).or_else(|| find_choose_tag(amount))
        }
        Expr::Extend { value, .. }
        | Expr::ZeroExtend { value, .. }
        | Expr::SignExtend { value, .. } => find_choose_tag(value),
        Expr::ExtendReg { register, shift, .. } => {
            find_choose_tag(register).or_else(|| find_choose_tag(shift))
        }
        Expr::Select {
            condition,
            when_true,
            when_false,
..
        } => find_choose_tag(condition)
            .or_else(|| find_choose_tag(when_true))
            .or_else(|| find_choose_tag(when_false)),
        Expr::TupleElement { value, .. }
        | Expr::BitTest { value, .. }
        | Expr::Extract { value, .. } => find_choose_tag(value),
        Expr::Concat { high, low, .. } => {
            find_choose_tag(high).or_else(|| find_choose_tag(low))
        }
        Expr::Replicate { value, .. } => find_choose_tag(value),
        Expr::AddWithCarry { left, right, .. } => {
            find_choose_tag(left).or_else(|| find_choose_tag(right))
        }
        _ => None,
    }
}

fn find_add_with_carry(expr: &Expr) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match expr {
        Expr::AddWithCarry { left, right, carry, .. } => Some((left, right, carry)),
        Expr::Select {
            when_true,
            when_false,
..
        } => find_add_with_carry(when_true).or_else(|| find_add_with_carry(when_false)),
        Expr::Concat { high, low, .. } => {
            find_add_with_carry(high).or_else(|| find_add_with_carry(low))
        }
        Expr::TupleElement { value, .. }
        | Expr::Extract { value, .. }
        | Expr::Unary { value, .. } => find_add_with_carry(value),
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
fn subg_validates() {
    let instruction = subg_instruction();

    validate_instruction(&instruction).expect("SUBG should validate");
}

#[test]
fn subg_requires_mte_feature() {
    let instruction = subg_instruction();

    assert_eq!(instruction.required_features, vec!["FEAT_MTE".to_string()]);

    let decode = instruction.decode.expect("SUBG needs decode bindings");
    assert_eq!(decode.statements.len(), 4);

    // offset = LSL(imm6, LOG2_TAG_GRANULE=4): 16-byte granule scaling.
    let has_scaled_offset = decode.statements.iter().any(|s| {
        matches!(
            s,
            DecodeStmt::Let {
                value: DecodeExpr::ShiftLeft { amount: 4, .. },
..
            }
        )
    });
    assert!(has_scaled_offset, "decode must scale imm6 by tag granule");
}

#[test]
fn subg_selects_non_excluded_tag() {
    let instruction = subg_instruction();
    let value = write_value(&instruction);

    let (start_tag, tag_offset, gcr) =
        find_choose_tag(value).expect("tagged result must contain ChooseTag");

    // start_tag = address<59:56>.
    let Expr::Extract { lsb: 56, width, .. } = start_tag.as_ref() else {
        panic!("start_tag must extract bits 59:56");
    };
    assert_eq!(*width, BitWidth::W4);

    // tag_offset = imm4.
    assert!(matches!(
        tag_offset.as_ref(),
        Expr::Const { value: 5, ty: BitWidth::W4 }
    ));

    // Exclusion mask comes from the whole GCR_EL1 system register.
    assert!(matches!(
        gcr.as_ref(),
        Expr::ReadSpecial {
            reg: SpecialReg::GcrEl1,
            width: BitWidth::W64
        }
    ));
}

#[test]
fn subg_subtracts_scaled_offset() {
    let instruction = subg_instruction();
    let value = write_value(&instruction);

    let (_, right, carry) =
        find_add_with_carry(value).expect("tagged result must contain AddWithCarry");

    // Subtraction uses carry-in = 1 and inverts the subtrahend.
    assert!(matches!(carry.as_ref(), Expr::Bool(true)));

    // Right operand = NOT(LSL(ZeroExtend{64}(imm6), 4)).
    let Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: shifted,
..
    } = right.as_ref()
    else {
        panic!("subtrahend must be bitwise-inverted");
    };

    let Expr::Shift {
        kind: ShiftKind::Lsl,
        value,
        amount,
        ty: BitWidth::W64,
    } = shifted.as_ref()
    else {
        panic!("offset must be a 64-bit logical left shift");
    };
    assert!(matches!(
        value.as_ref(),
        Expr::ZeroExtend { to: BitWidth::W64, .. }
    ));
    assert!(matches!(
        amount.as_ref(),
        Expr::Const { value: 4, .. }
    ));
}

#[test]
fn subg_inserts_tag_into_bits_59_56() {
    let instruction = subg_instruction();
    let value = write_value(&instruction);

    // Outer: result<63:60> : inner, total 64 bits.
    let Expr::Concat { high, low: inner, width: BitWidth::W64 } = value else {
        panic!("tagged result must be a 64-bit concat");
    };
    let Expr::Extract { lsb: 60, width: BitWidth::W4, .. } = high.as_ref() else {
        panic!("high part must preserve bits 63:60");
    };

    // Inner: rtag : result<55:0>, total 60 bits.
    let Expr::Concat {
        high: rtag,
        low,
        width,
    } = inner.as_ref()
    else {
        panic!("inner must concat tag over low bits");
    };
    assert_eq!(*width, BitWidth::new(60).unwrap());
    assert!(matches!(rtag.as_ref(), Expr::ChooseTag { .. }));
    let Expr::Extract { lsb: 0, width, .. } = low.as_ref() else {
        panic!("low part must preserve bits 55:0");
    };
    assert_eq!(*width, BitWidth::new(56).unwrap());
}

#[test]
fn subg_writes_sp_or_register() {
    let instruction = subg_instruction();

    let Stmt::If {
        condition,
        then_body,
        else_body,
    } = &instruction.semantics[0]
    else {
        panic!("expected If for conditional destination");
    };

    assert!(matches!(
        condition,
        Expr::Binary { op: semantic_ir::BinOp::Eq, .. }
    ));
    assert!(matches!(then_body[0], Stmt::WriteSP { .. }));
    assert!(matches!(
        else_body[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}
