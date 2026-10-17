/// CBH<cc> (Compare halfwords and branch) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cbhcc_regs.md
/// Stanford (arm64_md_no_nav_clean/cbhcc_regs.md):
///   Decode:
///     if !IsFeatureImplemented(FEAT_CMPBR) then EndOfDecode(Decode_UNDEF); end;
///     datasize = 8 << UInt(H); // H=1 for halfwords -> 16 bits
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
///     operand1 = X[t, 16];
///     operand2 = X[m, 16];
///     cond = Compare(op, operand1, operand2);
///     if cond then BranchTo(PC64() + offset);
///
/// Aliases (canonical target is CBH<cc> with swapped operands):
///   - CBHLE Rt, Rm, label == CBHGE Rm, Rt, label
///   - CBHLO Rt, Rm, label == CBHHI Rm, Rt, label
///   - CBHLS Rt, Rm, label == CBHHS Rm, Rt, label
///   - CBHLT Rt, Rm, label == CBHGT Rm, Rt, label
///
/// No IR extension: reuses existing `BinOp` comparison operators on W16 operands,
/// `SpecialReg::Pc64`, and `Stmt::BranchConditional`.
/// Requires `FEAT_CMPBR`.
use semantic_ir::{
    BinOp, BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CbhCondition {
    GtSigned,
    GeSigned,
    GtUnsigned,
    GeUnsigned,
    Eq,
    Ne,
}

impl CbhCondition {
    fn to_bin_op(self) -> BinOp {
        match self {
            CbhCondition::GtSigned => BinOp::GtSigned,
            CbhCondition::GeSigned => BinOp::GeSigned,
            CbhCondition::GtUnsigned => BinOp::GtUnsigned,
            CbhCondition::GeUnsigned => BinOp::GeUnsigned,
            CbhCondition::Eq => BinOp::Eq,
            CbhCondition::Ne => BinOp::Ne,
        }
    }

    fn mnemonic(self) -> &'static str {
        match self {
            CbhCondition::GtSigned => "CBHGT",
            CbhCondition::GeSigned => "CBHGE",
            CbhCondition::GtUnsigned => "CBHHI",
            CbhCondition::GeUnsigned => "CBHHS",
            CbhCondition::Eq => "CBHEQ",
            CbhCondition::Ne => "CBHNE",
        }
    }
}

fn cbhcc_instruction(cond: CbhCondition, t: u8, m: u8, offset_val: u128) -> Instruction {
    let reg_t = Reg::w(t).expect("valid w reg");
    let reg_m = Reg::w(m).expect("valid w reg");

    let operand1 = Expr::ReadReg {
        reg: reg_t,
        width: BitWidth::W16,
    };

    let operand2 = Expr::ReadReg {
        reg: reg_m,
        width: BitWidth::W16,
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
            encoding: Some("Compare halfwords and branch".to_string()),
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
            source_path: "cbhcc_regs.md".to_string(),
            source_sha256:
                "cfa404e32f260e2f588be803966aee89eb43101baaf1cf2ebd44cb6361014a96".to_string(),
            clean_sha256:
                "cfa404e32f260e2f588be803966aee89eb43101baaf1cf2ebd44cb6361014a96".to_string(),
        },
    }
}

#[test]
fn cbhcc_all_conditions_validate() {
    let conditions = [
        CbhCondition::GtSigned,
        CbhCondition::GeSigned,
        CbhCondition::GtUnsigned,
        CbhCondition::GeUnsigned,
        CbhCondition::Eq,
        CbhCondition::Ne,
    ];

    for cond in conditions {
        let ins = cbhcc_instruction(cond, 2, 4, 0x80);
        validate_instruction(&ins).expect("CBH<cc> variant should validate");
    }
}

#[test]
fn cbhcc_requires_feat_cmpbr() {
    let ins = cbhcc_instruction(CbhCondition::Eq, 0, 1, 0x20);
    assert_eq!(ins.required_features, vec!["FEAT_CMPBR".to_string()]);
}

#[test]
fn cbhcc_condition_and_target_structure() {
    let ins = cbhcc_instruction(CbhCondition::GtSigned, 5, 6, 0x100);
    assert_eq!(ins.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &ins.semantics[0] else {
        panic!("expected BranchConditional");
    };

    let Expr::Binary { op, left, right, ty } = condition else {
        panic!("expected Binary condition");
    };
    assert_eq!(*op, BinOp::GtSigned);
    assert_eq!(*ty, ValueType::Bool);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::W(5), width: BitWidth::W16 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::W(6), width: BitWidth::W16 }));

    let Expr::Binary { op: target_op, left: target_left, right: target_right, .. } = target else {
        panic!("expected Binary target");
    };
    assert_eq!(*target_op, BinOp::Add);
    assert!(matches!(target_left.as_ref(), Expr::ReadSpecial { reg: SpecialReg::Pc64, width: BitWidth::W64 }));
    assert!(matches!(target_right.as_ref(), Expr::Const { value: 0x100, ty: BitWidth::W64 }));
}

#[test]
fn cbhcc_aliases_resolve_to_canonical_target() {
    // CBHLE Rt, Rm == CBHGE Rm, Rt
    let cbhle = cbhcc_instruction(CbhCondition::GeSigned, 4, 2, 0x40);
    validate_instruction(&cbhle).expect("CBHLE alias should validate");

    // CBHLO Rt, Rm == CBHHI Rm, Rt
    let cbhlo = cbhcc_instruction(CbhCondition::GtUnsigned, 4, 2, 0x40);
    validate_instruction(&cbhlo).expect("CBHLO alias should validate");

    // CBHLS Rt, Rm == CBHHS Rm, Rt
    let cbhls = cbhcc_instruction(CbhCondition::GeUnsigned, 4, 2, 0x40);
    validate_instruction(&cbhls).expect("CBHLS alias should validate");

    // CBHLT Rt, Rm == CBHGT Rm, Rt
    let cbhlt = cbhcc_instruction(CbhCondition::GtSigned, 4, 2, 0x40);
    validate_instruction(&cbhlt).expect("CBHLT alias should validate");
}

#[test]
fn cbhcc_decode_binds_fields() {
    let ins = cbhcc_instruction(CbhCondition::Ne, 1, 2, 0x40);
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