/// BFM (Bitfield Move) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bfm.md
/// Stanford (arm64_md_no_nav_clean/bfm.md):
///   dst = Xd; src = Xn; r = immr; s = imms
///   (wmask, tmask) = DecodeBitMasks(N, imms, immr, FALSE)
///   bot = ROR(src, r) AND wmask
///   Xd = (dst AND NOT(tmask)) OR (bot AND tmask)
///
/// Key difference from UBFM: destination bits outside bitfield are
/// preserved (merged with original dst), not zeroed.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, ShiftKind, Stmt, ValueType, Width, validate_instruction,
};

fn bfm_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let src = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let dst = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let imm6 = BitWidth::new(6).unwrap();
    let r = Expr::Const {
        value: 8,
        ty: imm6,
    };
    let _s = Expr::Const {
        value: 15,
        ty: imm6,
    };

    // wmask: selects the bitfield from rotated source
    // tmask: selects where to place the bitfield in destination
    let wmask = Expr::Const {
        value: 0x000000000000FFFFu128,
        ty: BitWidth::W64,
    };
    let tmask = Expr::Const {
        value: 0x000000000000FFFFu128,
        ty: BitWidth::W64,
    };

    // bot = ROR(src, r) AND wmask
    let ror_src = Expr::Shift {
        kind: ShiftKind::Ror,
        value: Box::new(src),
        amount: Box::new(r),
        ty: BitWidth::W64,
    };

    let bot = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(ror_src),
        right: Box::new(wmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // result = (dst AND NOT(tmask)) OR (bot AND tmask)
    let not_tmask = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(tmask.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let dst_preserved = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(dst),
        right: Box::new(not_tmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let bot_masked = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(bot),
        right: Box::new(tmask),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(dst_preserved),
        right: Box::new(bot_masked),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFM".to_string(),
            encoding: Some("Bitfield move".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
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
                    name: "datasize".to_string(),
                    value: DecodeExpr::ShiftLeft {
                        value: Box::new(DecodeExpr::UInt("sf".to_string())),
                        amount: 5,
                    },
                },
                DecodeStmt::Let {
                    name: "r".to_string(),
                    value: DecodeExpr::UInt("immr".to_string()),
                },
                DecodeStmt::Let {
                    name: "s".to_string(),
                    value: DecodeExpr::UInt("imms".to_string()),
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "bfm.md".to_string(),
            source_sha256:
                "02c51b732b6132efc50d8e6f261fb0e5814d347443d1366bc8a403355104c060".to_string(),
            clean_sha256:
                "4ce7de58d3e872ed8e845c126b9a2cf3ac20f3ad9ada77e547f281f29355e532".to_string(),
        },
    }
}

#[test]
fn bfm_validates() {
    let instruction = bfm_instruction();
    validate_instruction(&instruction).expect("BFM should validate");
}

#[test]
fn bfm_writes_result() {
    let instruction = bfm_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}

#[test]
fn bfm_preserves_dst_bits_outside_bitfield() {
    let instruction = bfm_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected register write");
    };
    // result = (dst AND NOT(tmask)) OR (bot AND tmask)
    let Expr::Binary { op: op1, left, right, .. } = value else {
        panic!("expected Binary Or for final result");
    };
    assert_eq!(*op1, semantic_ir::BinOp::Or);

    // left = dst AND NOT(tmask) - preserves destination outside bitfield
    let Expr::Binary { op: op2, left: dst_left, right: dst_right, .. } = left.as_ref() else {
        panic!("expected Binary And for dst preservation");
    };
    assert_eq!(*op2, semantic_ir::BinOp::And);
    assert!(matches!(dst_left.as_ref(), Expr::ReadReg { reg: Reg::X(1), .. }));
    let Expr::Unary { op: unary_op, value, .. } = dst_right.as_ref() else {
        panic!("expected Unary Not for NOT(tmask)");
    };
    assert_eq!(*unary_op, semantic_ir::UnaryOp::Not);
    assert!(matches!(value.as_ref(), Expr::Const { .. }));

    // right = bot AND tmask - the bitfield from rotated source
    // where bot = ROR(src, r) AND wmask
    let Expr::Binary { op: op3, left: bot_masked_left, right: bot_masked_right, .. } = right.as_ref() else {
        panic!("expected Binary And for bot masked");
    };
    assert_eq!(*op3, semantic_ir::BinOp::And);
    assert!(matches!(bot_masked_right.as_ref(), Expr::Const { .. }));

    // bot_masked_left = bot = ROR(src, r) AND wmask
    let Expr::Binary { op: op4, left: bot_left, right: bot_right, .. } = bot_masked_left.as_ref() else {
        panic!("expected Binary And for bot (ROR AND wmask)");
    };
    assert_eq!(*op4, semantic_ir::BinOp::And);
    assert!(matches!(bot_right.as_ref(), Expr::Const { .. }));

    // bot_left = ROR(src, r)
    let Expr::Shift { kind, value: ror_value, amount, .. } = bot_left.as_ref() else {
        panic!("expected Shift ROR for bot");
    };
    assert_eq!(*kind, ShiftKind::Ror);
    assert!(matches!(ror_value.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
    assert!(matches!(amount.as_ref(), Expr::Const { value: 8, .. }));
}