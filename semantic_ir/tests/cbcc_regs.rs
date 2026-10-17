/// CB<cc> (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cbcc_regs.md
/// Stanford (arm64_md_no_nav_clean/cbcc_regs.md):
///   Decode:
///     if !IsFeatureImplemented(FEAT_CMPBR) then EndOfDecode(Decode_UNDEF); end;
///     datasize = 32 << UInt(sf);
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
///     operand1 = X[t, datasize];
///     operand2 = X[m, datasize];
///     cond = Compare(op, operand1, operand2);
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
enum CbRegCondition {
    GtSigned,
    GeSigned,
    GtUnsigned,
    GeUnsigned,
    Eq,
    Ne,
}

impl CbRegCondition {
    fn to_bin_op(self) -> BinOp {
        match self {
            CbRegCondition::GtSigned => BinOp::GtSigned,
            CbRegCondition::GeSigned => BinOp::GeSigned,
            CbRegCondition::GtUnsigned => BinOp::GtUnsigned,
            CbRegCondition::GeUnsigned => BinOp::GeUnsigned,
            CbRegCondition::Eq => BinOp::Eq,
            CbRegCondition::Ne => BinOp::Ne,
        }
    }

    fn mnemonic(self) -> &'static str {
        match self {
            CbRegCondition::GtSigned => "CBGT",
            CbRegCondition::GeSigned => "CBGE",
            CbRegCondition::GtUnsigned => "CBHI",
            CbRegCondition::GeUnsigned => "CBHS",
            CbRegCondition::Eq => "CBEQ",
            CbRegCondition::Ne => "CBNE",
        }
    }
}

fn cbcc_regs_instruction(
    cond: CbRegCondition,
    is_64bit: bool,
    t: u8,
    m: u8,
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

    let reg_m = if is_64bit {
        Reg::x(m).expect("valid x reg")
    } else {
        Reg::w(m).expect("valid w reg")
    };

    let operand1 = Expr::ReadReg {
        reg: reg_t,
        width,
    };

    let operand2 = Expr::ReadReg {
        reg: reg_m,
        width,
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
            encoding: Some("Compare registers and branch".to_string()),
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
            source_path: "cbcc_regs.md".to_string(),
            source_sha256:
                "0fcc28ca3db0930db346950a96ce8bd1c02064efb8f5f800fcc570eb326f0ef3".to_string(),
            clean_sha256:
                "7d328e37cf9b01ac8eafdb89c703a5a82d81c7f5add20c604b758712fa558935".to_string(),
        },
    }
}

#[test]
fn cbcc_regs_all_conditions_validate() {
    let conditions = [
        CbRegCondition::GtSigned,
        CbRegCondition::GeSigned,
        CbRegCondition::GtUnsigned,
        CbRegCondition::GeUnsigned,
        CbRegCondition::Eq,
        CbRegCondition::Ne,
    ];

    for cond in conditions {
        for is_64bit in [false, true] {
            let ins = cbcc_regs_instruction(cond, is_64bit, 3, 5, 0x100);
            validate_instruction(&ins).expect("CB<cc> (regs) variant should validate");
        }
    }
}

#[test]
fn cbcc_regs_requires_feat_cmpbr() {
    let ins = cbcc_regs_instruction(CbRegCondition::Eq, false, 0, 1, 0x20);
    assert_eq!(ins.required_features, vec!["FEAT_CMPBR".to_string()]);
}

#[test]
fn cbcc_regs_condition_and_target_structure() {
    let ins = cbcc_regs_instruction(CbRegCondition::GeSigned, true, 5, 6, 0x80);
    assert_eq!(ins.semantics.len(), 1);

    let Stmt::BranchConditional { condition, target } = &ins.semantics[0] else {
        panic!("expected BranchConditional");
    };

    let Expr::Binary { op, left, right, ty } = condition else {
        panic!("expected Binary condition");
    };
    assert_eq!(*op, BinOp::GeSigned);
    assert_eq!(*ty, ValueType::Bool);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(5), width: BitWidth::W64 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::X(6), width: BitWidth::W64 }));

    let Expr::Binary { op: target_op, left: target_left, right: target_right, .. } = target else {
        panic!("expected Binary target");
    };
    assert_eq!(*target_op, BinOp::Add);
    assert!(matches!(target_left.as_ref(), Expr::ReadSpecial { reg: SpecialReg::Pc64, width: BitWidth::W64 }));
    assert!(matches!(target_right.as_ref(), Expr::Const { value: 0x80, ty: BitWidth::W64 }));
}

#[test]
fn cbcc_regs_decode_binds_fields() {
    let ins = cbcc_regs_instruction(CbRegCondition::Ne, false, 1, 2, 0x40);
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
