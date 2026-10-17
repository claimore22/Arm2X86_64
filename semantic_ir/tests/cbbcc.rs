/// CBB<cc> (Compare bytes and branch) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cbbcc_regs.md
/// Stanford (arm64_md_no_nav_clean/cbbcc_regs.md):
///   Decode:
///     if !IsFeatureImplemented(FEAT_CMPBR) then EndOfDecode(Decode_UNDEF); end;
///     datasize = 8 << UInt(H); // H=0 for bytes -> 8 bits
///     t = UInt(Rt); m = UInt(Rm);
///     offset = SignExtend(imm9::'00');
///     case cc of
///       when '000' => op = Cmp_GT; unsigned = FALSE;
///       when '001' => op = Cmp_GE; unsigned = FALSE;
///       when '010' => op = Cmp_GT; unsigned = TRUE;
///       when '011' => op = Cmp_GE; unsigned = TRUE;
///       when '110' => op = Cmp_EQ; unsigned = TRUE;
///       when '111' => op = Cmp_NE; unsigned = TRUE;
///       otherwise  => EndOfDecode(Decode_UNDEF);
///   Operation:
///     operand1 = X[t, 8];
///     operand2 = X[m, 8];
///     cond = Compare(op, operand1, operand2);
///     if cond then BranchTo(PC64() + offset);
///
/// Aliases (canonical target is CBB<cc> with swapped operands):
///   - CBBLE Rt, Rm, label == CBBGE Rm, Rt, label
///   - CBBLO Rt, Rm, label == CBBHI Rm, Rt, label
///   - CBBLS Rt, Rm, label == CBBHS Rm, Rt, label
///   - CBBLT Rt, Rm, label == CBBGT Rm, Rt, label
///
/// No IR extension: reuses existing `BinOp` comparison operators on W8 operands,
/// `SpecialReg::Pc64`, and `Stmt::BranchConditional`.
/// Requires `FEAT_CMPBR`.
use semantic_ir::{
    BinOp, BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CbbCondition {
    GtSigned,
    GeSigned,
    GtUnsigned,
    GeUnsigned,
    Eq,
    Ne,
}

impl CbbCondition {
    fn to_bin_op(self) -> BinOp {
        match self {
            CbbCondition::GtSigned => BinOp::GtSigned,
            CbbCondition::GeSigned => BinOp::GeSigned,
            CbbCondition::GtUnsigned => BinOp::GtUnsigned,
            CbbCondition::GeUnsigned => BinOp::GeUnsigned,
            CbbCondition::Eq => BinOp::Eq,
            CbbCondition::Ne => BinOp::Ne,
        }
    }

    fn mnemonic(self) -> &'static str {
        match self {
            CbbCondition::GtSigned => "CBBGT",
            CbbCondition::GeSigned => "CBBGE",
            CbbCondition::GtUnsigned => "CBBHI",
            CbbCondition::GeUnsigned => "CBBHS",
            CbbCondition::Eq => "CBBEQ",
            CbbCondition::Ne => "CBBNE",
        }
    }
}

fn cbbcc_instruction(cond: CbbCondition, t: u8, m: u8, offset_val: u128) -> Instruction {
    let reg_t = Reg::w(t).expect("valid w reg");
    let reg_m = Reg::w(m).expect("valid w reg");

    let operand1 = Expr::ReadReg {
        reg: reg_t,
        width: BitWidth::W8,
    };

    let operand2 = Expr::ReadReg {
        reg: reg_m,
        width: BitWidth::W8,
    };

    let condition = Expr::Binary {
        op: cond.to_bin_op(),
        left: Box::new(operand1),
        right: Box::new(operand2),
        ty: ValueType::Bool,
    };

    let offset = Expr::Const {
        value: offset_val,
        ty: BitWidth::W64,
    };

    let target = Expr::Binary {
        op: BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc64,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::BranchConditional { condition, target }];

    Instruction {
        id: InstructionId {
            mnemonic: cond.mnemonic().to_string(),
            encoding: Some("Compare bytes and branch".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CMPBR".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "t".to_string(),
                    value: DecodeExpr::UInt("Rt".to_string()),
                },
                DecodeStmt::Let {
                    name: "m".to_string(),
                    value: DecodeExpr::UInt("Rm".to_string()),
                },
                DecodeStmt::Let {
                    name: "offset".to_string(),
                    value: DecodeExpr::ShiftLeft {
                        value: Box::new(DecodeExpr::UInt("imm9".to_string())),
                        amount: 2,
                    },
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "cbbcc_regs.md".to_string(),
            source_sha256:
                "2b4fe0d967cf0d90957a524fcdd5891e90f181ab86e786591bf652d206fa378b".to_string(),
            clean_sha256:
                "8f223927a5fe4ea060285cf7bdbcce05ce9c81bf22829681f811ab03d8fce23f".to_string(),
        },
    }
}

#[test]
fn cbbcc_all_conditions_validate() {
    let conditions = [
        CbbCondition::GtSigned,
        CbbCondition::GeSigned,
        CbbCondition::GtUnsigned,
        CbbCondition::GeUnsigned,
        CbbCondition::Eq,
        CbbCondition::Ne,
    ];

    for cond in conditions {
        let ins = cbbcc_instruction(cond, 2, 4, 0x80);
        validate_instruction(&ins).expect("CBB<cc> variant should validate");
    }
}

#[test]
fn cbbcc_requires_feat_cmpbr() {
    let ins = cbbcc_instruction(CbbCondition::Eq, 0, 1, 0x20);
    assert_eq!(ins.required_features, vec!["FEAT_CMPBR".to_string()]);
}

#[test]
fn cbbcc_condition_and_target_structure() {
    let ins = cbbcc_instruction(CbbCondition::GtSigned, 5, 6, 0x100);
    assert_eq!(ins.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &ins.semantics[0] else {
        panic!("expected BranchConditional");
    };

    let Expr::Binary { op, left, right, ty } = condition else {
        panic!("expected Binary condition");
    };
    assert_eq!(*op, BinOp::GtSigned);
    assert_eq!(*ty, ValueType::Bool);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::W(5), width: BitWidth::W8 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::W(6), width: BitWidth::W8 }));

    let Expr::Binary { op: target_op, left: target_left, right: target_right, .. } = target else {
        panic!("expected Binary target");
    };
    assert_eq!(*target_op, BinOp::Add);
    assert!(matches!(target_left.as_ref(), Expr::ReadSpecial { reg: SpecialReg::Pc64, width: BitWidth::W64 }));
    assert!(matches!(target_right.as_ref(), Expr::Const { value: 0x100, ty: BitWidth::W64 }));
}

#[test]
fn cbbcc_aliases_resolve_to_canonical_target() {
    // CBBLE Rt, Rm == CBBGE Rm, Rt
    let cbble = cbbcc_instruction(CbbCondition::GeSigned, 4, 2, 0x40);
    validate_instruction(&cbble).expect("CBBLE alias should validate");

    // CBBLO Rt, Rm == CBBHI Rm, Rt
    let cbblo = cbbcc_instruction(CbbCondition::GtUnsigned, 4, 2, 0x40);
    validate_instruction(&cbblo).expect("CBBLO alias should validate");

    // CBBLS Rt, Rm == CBBHS Rm, Rt
    let cbbls = cbbcc_instruction(CbbCondition::GeUnsigned, 4, 2, 0x40);
    validate_instruction(&cbbls).expect("CBBLS alias should validate");

    // CBBLT Rt, Rm == CBBGT Rm, Rt
    let cbblt = cbbcc_instruction(CbbCondition::GtSigned, 4, 2, 0x40);
    validate_instruction(&cbblt).expect("CBBLT alias should validate");
}

#[test]
fn cbbcc_decode_binds_fields() {
    let ins = cbbcc_instruction(CbbCondition::Ne, 1, 2, 0x40);
    let decode = ins.decode.expect("decode spec required");
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "t"
    )));
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "m"
    )));
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "offset"
    )));
}
