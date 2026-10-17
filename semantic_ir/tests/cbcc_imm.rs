/// CB<cc> (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cbcc_imm.md
/// Stanford (arm64_md_no_nav_clean/cbcc_imm.md):
///   Decode:
///     if !IsFeatureImplemented(FEAT_CMPBR) then EndOfDecode(Decode_UNDEF); end;
///     datasize = 32 << UInt(sf);
///     t = UInt(Rt);
///     offset = SignExtend(imm9::'00');
///     case cc of
///       when '000' => op = Cmp_GT; unsigned = FALSE;
///       when '001' => op = Cmp_LT; unsigned = FALSE;
///       when '010' => op = Cmp_GT; unsigned = TRUE;
///       when '011' => op = Cmp_LT; unsigned = TRUE;
///       when '110' => op = Cmp_EQ; unsigned = TRUE;
///       when '111' => op = Cmp_NE; unsigned = TRUE;
///       otherwise  => EndOfDecode(Decode_UNDEF);
///     value2 = UInt(imm6);
///   Operation:
///     operand1 = X[t, datasize];
///     cond = Compare(op, operand1, value2);
///     if cond then BranchTo(PC64() + offset);
///
/// No IR extension: reuses existing `BinOp` comparison operators,
/// `SpecialReg::Pc64`, and `Stmt::BranchConditional`.
/// Requires `FEAT_CMPBR`.
use semantic_ir::{
    BinOp, BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, Reg, SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CbCondition {
    GtSigned,
    LtSigned,
    GtUnsigned,
    LtUnsigned,
    Eq,
    Ne,
}

impl CbCondition {
    fn to_bin_op(self) -> BinOp {
        match self {
            CbCondition::GtSigned => BinOp::GtSigned,
            CbCondition::LtSigned => BinOp::LtSigned,
            CbCondition::GtUnsigned => BinOp::GtUnsigned,
            CbCondition::LtUnsigned => BinOp::LtUnsigned,
            CbCondition::Eq => BinOp::Eq,
            CbCondition::Ne => BinOp::Ne,
        }
    }

    fn mnemonic(self) -> &'static str {
        match self {
            CbCondition::GtSigned => "CBGT",
            CbCondition::LtSigned => "CBLT",
            CbCondition::GtUnsigned => "CBHI",
            CbCondition::LtUnsigned => "CBLO",
            CbCondition::Eq => "CBEQ",
            CbCondition::Ne => "CBNE",
        }
    }
}

fn cbcc_imm_instruction(
    cond: CbCondition,
    is_64bit: bool,
    t: u8,
    imm6: u128,
    offset_val: u128,
) -> Instruction {
    let width = if is_64bit {
        BitWidth::W64
    } else {
        BitWidth::W32
    };

    let reg_t = if is_64bit {
        Reg::x(t).expect("valid x reg")
    } else {
        Reg::w(t).expect("valid w reg")
    };

    let operand1 = Expr::ReadReg {
        reg: reg_t,
        width,
    };

    let value2 = Expr::Const {
        value: imm6,
        ty: width,
    };

    let condition = Expr::Binary {
        op: cond.to_bin_op(),
        left: Box::new(operand1),
        right: Box::new(value2),
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
            encoding: Some("Compare register with immediate and branch".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CMPBR".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "sf".to_string(),
                    value: DecodeExpr::Immediate(if is_64bit { 1 } else { 0 }),
                },
                DecodeStmt::Let {
                    name: "t".to_string(),
                    value: DecodeExpr::UInt("Rt".to_string()),
                },
                DecodeStmt::Let {
                    name: "imm6".to_string(),
                    value: DecodeExpr::UInt("imm6".to_string()),
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
            source_path: "cbcc_imm.md".to_string(),
            source_sha256:
                "6a170b897a338811fe57d5b4db661adf3a16153126f10f1a030ebf80a19debfc".to_string(),
            clean_sha256:
                "2edcaeea98c0ce1e6bbd23a2979f635d123341c98154ec868c2ab96d946ef4f1".to_string(),
        },
    }
}

#[test]
fn cbcc_imm_all_conditions_validate() {
    let conditions = [
        CbCondition::GtSigned,
        CbCondition::LtSigned,
        CbCondition::GtUnsigned,
        CbCondition::LtUnsigned,
        CbCondition::Eq,
        CbCondition::Ne,
    ];

    for cond in conditions {
        for is_64bit in [false, true] {
            let ins = cbcc_imm_instruction(cond, is_64bit, 3, 42, 0x100);
            validate_instruction(&ins).expect("CB<cc> (imm) variant should validate");
        }
    }
}

#[test]
fn cbcc_imm_requires_feat_cmpbr() {
    let ins = cbcc_imm_instruction(CbCondition::Eq, false, 0, 10, 0x20);
    assert_eq!(ins.required_features, vec!["FEAT_CMPBR".to_string()]);
}

#[test]
fn cbcc_imm_condition_and_target_structure() {
    let ins = cbcc_imm_instruction(CbCondition::GtSigned, true, 5, 20, 0x80);
    assert_eq!(ins.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &ins.semantics[0] else {
        panic!("expected BranchConditional");
    };

    let Expr::Binary { op, left, right, ty } = condition else {
        panic!("expected Binary condition");
    };
    assert_eq!(*op, BinOp::GtSigned);
    assert_eq!(*ty, ValueType::Bool);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(5), width: BitWidth::W64 }));
    assert!(matches!(right.as_ref(), Expr::Const { value: 20, ty: BitWidth::W64 }));

    let Expr::Binary { op: target_op, left: target_left, right: target_right, .. } = target else {
        panic!("expected Binary target");
    };
    assert_eq!(*target_op, BinOp::Add);
    assert!(matches!(target_left.as_ref(), Expr::ReadSpecial { reg: SpecialReg::Pc64, width: BitWidth::W64 }));
    assert!(matches!(target_right.as_ref(), Expr::Const { value: 0x80, ty: BitWidth::W64 }));
}

#[test]
fn cbcc_imm_decode_binds_fields() {
    let ins = cbcc_imm_instruction(CbCondition::Ne, false, 1, 5, 0x40);
    let decode = ins.decode.expect("decode spec required");
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "t"
    )));
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "imm6"
    )));
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "offset"
    )));
}
